#![allow(non_snake_case, non_upper_case_globals)]
use std::sync::Arc;

use common::resampler::{calc_hertz, resample};
use crossbeam::channel::{unbounded, Receiver, Sender};
use cyma::bus::{Bus, MonoBus};
use engine::{Adsr, Voice};
use instrument::Instrument;
use nih_plug::prelude::*;
use presets::Presets;
use vizia_plug::ViziaState;

mod editor;
pub mod instrument;
mod presets;

const DEFAULT_ATTACK_S: f32 = 0.01;
const DEFAULT_DECAY_S: f32 = 0.1;
const DEFAULT_SUSTAIN_LEVEL: f32 = 1.0;
const DEFAULT_RELEASE_S: f32 = 0.2;
const ORIGINAL_SAMPLE_RATE: f32 = 44100.0;

enum Command {
    LoadPreset(Presets),
}

struct Orchestron {
    params: Arc<OrchestronParams>,
    voices: Vec<Voice>,
    instrument: Instrument,
    sample_rate: f32,
    adsr: Adsr,
    bus: Arc<MonoBus>,
    /// Sends/Receives commands to/on the audio thread.
    command_channel: (Sender<Command>, Receiver<Command>),
}

#[derive(Params)]
struct OrchestronParams {
    #[id = "gain"]
    pub gain: FloatParam,
    #[id = "attack"]
    pub attack: FloatParam,
    #[id = "decay"]
    pub decay: FloatParam,
    #[id = "sustain"]
    pub sustain: FloatParam,
    #[id = "release"]
    pub release: FloatParam,
    #[id = "preset"]
    pub preset: EnumParam<Presets>,
    #[persist = "editor-state"]
    editor_state: Arc<ViziaState>,
}

impl Default for Orchestron {
    fn default() -> Self {
        let command_channel = unbounded();

        Self {
            params: Arc::new(OrchestronParams::new(command_channel.0.clone())),
            voices: Vec::new(),
            instrument: Instrument::default(),
            sample_rate: ORIGINAL_SAMPLE_RATE,
            adsr: Adsr::new(ORIGINAL_SAMPLE_RATE),
            bus: Default::default(),
            command_channel,
        }
    }
}

impl OrchestronParams {
    fn new(command_sender: Sender<Command>) -> Self {
        Self {
            gain: FloatParam::new(
                "Gain",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
                    // This makes the range appear as if it was linear when displaying the values as
                    // decibels
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
            attack: FloatParam::new(
                "Attack",
                DEFAULT_ATTACK_S,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 2.0,
                    factor: 0.25,
                },
            )
            .with_unit(" s"),
            decay: FloatParam::new(
                "Decay",
                DEFAULT_DECAY_S,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 2.0,
                    factor: 0.25,
                },
            )
            .with_unit(" s"),
            sustain: FloatParam::new(
                "Sustain",
                DEFAULT_SUSTAIN_LEVEL,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_unit(" level")
            .with_value_to_string(formatters::v2s_f32_percentage(2)),
            release: FloatParam::new(
                "Release",
                DEFAULT_RELEASE_S,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 5.0,
                    factor: 0.25,
                },
            )
            .with_unit(" s"),
            preset: EnumParam::new("Preset", Presets::default()).with_callback(Arc::new(
                move |preset_value| {
                    command_sender.send(Command::LoadPreset(preset_value)).ok();
                },
            )),
            editor_state: editor::default_state(),
        }
    }
}

impl Plugin for Orchestron {
    const NAME: &'static str = "Orchestron";
    const VENDOR: &'static str = env!("PKG_VENDOR");
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = env!("PKG_EMAIL");

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),

        aux_input_ports: &[],
        aux_output_ports: &[],

        names: PortNames::const_default(),
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::Basic;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();

    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(
            self.params.clone(),
            self.params.editor_state.clone(),
            self.bus.clone(),
        )
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        nih_log!("Initializing Orchestron plugin.");
        self.sample_rate = buffer_config.sample_rate;
        self.bus.set_sample_rate(self.sample_rate);
        self.adsr = Adsr::new(self.sample_rate);

        self.command_channel
            .0
            .send(Command::LoadPreset(self.params.preset.value()))
            .ok();
        nih_log!("Initialization complete. Preset load task signaled for process loop.");

        true
    }

    fn reset(&mut self) {
        self.voices.clear();
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        while let Ok(command) = self.command_channel.1.try_recv() {
            match command {
                Command::LoadPreset(preset) => {
                    self.load_preset(preset);
                }
            }
        }

        let mut next_event = context.next_event();

        self.adsr.set_parameters(
            self.params.attack.value(),
            self.params.decay.value(),
            self.params.sustain.value(),
            self.params.release.value(),
        );

        for (sample_id, channel_samples) in buffer.iter_samples().enumerate() {
            while let Some(event) = next_event {
                if event.timing() > sample_id as u32 {
                    break;
                }

                match event {
                    NoteEvent::NoteOn { note, velocity, .. } => {
                        let playback_rate = calc_hertz(self.sample_rate, 53 - (note as i32));

                        let resampled = resample(&self.instrument.sample, 44100.0, playback_rate);

                        let new_voice = Voice::new(
                            Arc::new(resampled),
                            note,
                            velocity,
                            self.adsr.clone(),
                            true,
                        );

                        self.voices.push(new_voice);
                    }

                    NoteEvent::NoteOff { note, .. } => {
                        self.voices
                            .iter_mut()
                            .filter(|v| v.matches_note(note))
                            .for_each(|v| v.note_off());
                    }

                    _ => (),
                }

                next_event = context.next_event();
            }

            let mut output_sample = 0.0;
            for voice in &mut self.voices {
                output_sample += voice.next_sample();
            }

            let gain = self.params.gain.smoothed.next();
            output_sample *= gain;

            for sample in channel_samples {
                *sample = output_sample;
            }

            self.voices.retain(|v| v.is_active());
        }

        if self.params.editor_state.is_open() {
            self.bus.send_buffer_summing(buffer);
        }

        ProcessStatus::Normal
    }
}

impl Orchestron {
    pub fn load_preset(&mut self, preset: Presets) {
        self.voices.clear();

        let instrument_data = preset.content().to_vec();
        // Spawning a thread to decode the instrument data.
        self.instrument = std::thread::spawn(move || Instrument::decode(instrument_data))
            .join()
            .expect("Failed to load preset on a different thread");
    }
}

impl ClapPlugin for Orchestron {
    const CLAP_ID: &'static str = config::clap_id!();
    const CLAP_DESCRIPTION: Option<&'static str> = None;
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = Some(Self::URL);

    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::Sampler, ClapFeature::Instrument];
}

impl Vst3Plugin for Orchestron {
    const VST3_CLASS_ID: [u8; 16] = config::vst3_id!();

    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Sampler, Vst3SubCategory::Instrument];
}

nih_export_clap!(Orchestron);
nih_export_vst3!(Orchestron);
