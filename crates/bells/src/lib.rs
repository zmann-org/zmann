#![allow(non_snake_case, non_upper_case_globals)]
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crossbeam::channel::{unbounded, Receiver, Sender};
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
const ORIGINAL_SAMPLE_RATE: f32 = 44100.0;

enum Task {
    LoadPreset { preset: Presets, sample_rate: f32 },
}

struct Bells {
    params: Arc<BellsParams>,
    voices: Vec<Voice>,
    instrument: Instrument,
    sample_rate: f32,
    adsr: Adsr,
    task_channel: (Sender<Instrument>, Receiver<Instrument>),
}

#[derive(Params)]
struct BellsParams {
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
    // This flag is used to signal the audio thread that the preset has changed.
    pub preset_change: Arc<AtomicBool>,
}

impl Default for Bells {
    fn default() -> Self {
        Self {
            params: Arc::new(BellsParams::default()),
            voices: Vec::new(),
            instrument: Instrument::default(),
            sample_rate: ORIGINAL_SAMPLE_RATE,
            adsr: Adsr::new(ORIGINAL_SAMPLE_RATE),
            task_channel: unbounded(),
        }
    }
}

impl Default for BellsParams {
    fn default() -> Self {
        let preset_change = Arc::new(AtomicBool::new(false));

        Self {
            gain: FloatParam::new(
                "Gain",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
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

impl Plugin for Bells {
    const NAME: &'static str = "Bells";
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

    type BackgroundTask = Task;

    fn task_executor(&mut self) -> TaskExecutor<Self> {
        let sender = self.task_channel.0.clone();
        Box::new(move |task| match task {
            Task::LoadPreset {
                preset,
                sample_rate,
            } => {
                nih_log!("Starting background task: LoadPreset for {:?}", preset);
                let instrument = Bells::load_instrument_data(preset, sample_rate);
                nih_log!("Finished loading instrument in background task.");
                if let Err(err) = sender.send(instrument) {
                    nih_log!(
                        "Failed to send loaded instrument from background task: {}",
                        err
                    );
                } else {
                    nih_log!("Successfully sent instrument to audio thread.");
                }
            }
        })
    }

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        nih_log!("Initializing Bells plugin.");
        self.sample_rate = buffer_config.sample_rate;
        self.adsr = Adsr::new(self.sample_rate);

        self.voices.clear();

        // Signal the `process` function to load the initial preset. This is
        // non-blocking.
        self.params.preset_change.store(true, Ordering::Relaxed);
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
        // Check if a new instrument has been loaded by a background task.
        if let Ok(new_instrument) = self.task_channel.1.try_recv() {
            nih_log!("New instrument received. Swapping and clearing voices.");
            self.instrument = new_instrument;
            self.voices.clear();
        }

        // Check if the preset has been changed (either from the GUI or from
        // initialize()).
        if self.params.preset_change.swap(false, Ordering::Relaxed) {
            nih_log!("Preset change detected. Clearing voices and starting background load.");
            self.instrument.clear();
            self.voices.clear();
            context.execute_background(Task::LoadPreset {
                preset: self.params.preset.value(),
                sample_rate: self.sample_rate,
            });
        }
        let mut next_event = context.next_event();

        // Update ADSR parameters from the plugin's state.
        self.adsr.set_parameters(
            self.params.attack.value(),
            self.params.decay.value(),
            self.params.sustain.value(),
            self.params.release.value(),
        );

        for (sample_id, channel_samples) in buffer.iter_samples().enumerate() {
            // Process MIDI events for this sample.
            while let Some(event) = next_event {
                if event.timing() > sample_id as u32 {
                    break;
                }
                match event {
                    NoteEvent::NoteOn { note, velocity, .. } => {
                        if let Some(data) = self.instrument.samples.get(&note) {
                            // Cloning the Arc is cheap (it just increments a reference count).
                            let new_voice = Voice::new(
                                Arc::clone(data),
                                note,
                                velocity,
                                self.adsr.clone(),
                                false,
                            );
                            self.voices.push(new_voice);
                        }
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

            // Get the smoothed gain value.
            let gain = self.params.gain.smoothed.next();

            // Sum the output of all active voices.
            let mut output_sample = 0.0;
            for voice in &mut self.voices {
                output_sample += voice.next_sample();
            }

            // Write the final sample to all channels.
            for sample in channel_samples {
                *sample = output_sample * gain;
            }
        }

        // Remove voices that are no longer active.
        self.voices.retain(|v| v.is_active());

        ProcessStatus::Normal
    }
}

impl Bells {
    fn load_instrument_data(preset: Presets, sample_rate: f32) -> Instrument {
        let instrument_data = preset.content().to_vec();

        // This part is now executed on a background thread.
        nih_log!("Decoding instrument data for {:?}.", preset);
        let mut instrument = Instrument::decode(instrument_data);
        nih_log!("Finished decoding.");

        if sample_rate != ORIGINAL_SAMPLE_RATE {
            nih_log!(
                "Resampling instrument from {} Hz to {} Hz.",
                ORIGINAL_SAMPLE_RATE,
                sample_rate
            );
            instrument.samples = instrument
                .samples
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        Arc::new(common::resampler::resample(
                            &v,
                            ORIGINAL_SAMPLE_RATE,
                            sample_rate,
                        )),
                    )
                })
                .collect();
            nih_log!("Finished resampling.");
        }

        instrument
    }
}

impl ClapPlugin for Bells {
    const CLAP_ID: &'static str = config::clap_id!();
    const CLAP_DESCRIPTION: Option<&'static str> = None;
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = Some(Self::URL);

    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::Sampler, ClapFeature::Instrument];
}

impl Vst3Plugin for Bells {
    const VST3_CLASS_ID: [u8; 16] = config::vst3_id!();

    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Sampler, Vst3SubCategory::Instrument];
}

nih_export_clap!(Bells);
nih_export_vst3!(Bells);
