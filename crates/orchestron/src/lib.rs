#![allow(non_snake_case, non_upper_case_globals)]
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use common::resampler::{calc_hertz, resample};
use engine::{Adsr, Voice};
use instrument::Instrument;
use nih_plug::prelude::*;
use presets::Presets;

pub mod instrument;
mod presets;

const DEFAULT_ATTACK_S: f32 = 0.01;
const DEFAULT_DECAY_S: f32 = 0.1;
const DEFAULT_SUSTAIN_LEVEL: f32 = 1.0;
const DEFAULT_RELEASE_S: f32 = 0.2;

struct Orchestron {
    params: Arc<OrchestronParams>,
    voices: Vec<Voice>,
    instrument: Instrument,
    sample_rate: f32,
    adsr: Adsr,
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
    pub preset_change: Arc<AtomicBool>,
}

impl Default for Orchestron {
    fn default() -> Self {
        let sample_rate: f32 = 44100.0;

        Self {
            params: Arc::new(OrchestronParams::default()),
            voices: Vec::new(),
            instrument: Instrument::default(),
            sample_rate,
            adsr: Adsr::new(sample_rate),
        }
    }
}

impl Default for OrchestronParams {
    fn default() -> Self {
        let preset_change = Arc::new(AtomicBool::new(false));

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
            preset: EnumParam::new("Preset", Presets::default()).with_callback({
                let preset_change = preset_change.clone();
                Arc::new(move |_| {
                    preset_change.store(true, Ordering::Relaxed);
                })
            }),
            preset_change,
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

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        self.sample_rate = buffer_config.sample_rate;
        if self.instrument.sample.is_empty() {
            self.load_preset(self.params.preset.value());
        }

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

        if self.params.preset_change.swap(false, Ordering::Relaxed)
            && self.instrument.name != self.params.preset.value().to_string()
        {
            self.load_preset(self.params.preset.value());
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
