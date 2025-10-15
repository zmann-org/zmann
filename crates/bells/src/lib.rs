#![allow(non_snake_case, non_upper_case_globals)]
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

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

struct Bells {
    params: Arc<BellsParams>,
    voices: Vec<Voice>,
    instrument: Instrument,
    sample_rate: f32,
    adsr: Adsr,
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
        let sample_rate = 44100.0;

        Self {
            params: Arc::new(BellsParams::default()),
            voices: Vec::new(),
            instrument: Instrument::default(),
            sample_rate,
            adsr: Adsr::new(sample_rate),
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
        self.adsr = Adsr::new(self.sample_rate);

        if self.instrument.samples.is_empty() {
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

        // Check if the preset has been changed on the GUI thread.
        if self.params.preset_change.swap(false, Ordering::Relaxed) {
            self.load_preset(self.params.preset.value());
        }

        ProcessStatus::Normal
    }
}

impl Bells {
    pub fn load_preset(&mut self, preset: Presets) {
        self.voices.clear();

        let instrument_data = preset.content().to_vec();
        // Spawning a thread to decode the instrument data.
        self.instrument = std::thread::spawn(move || Instrument::decode(instrument_data))
            .join()
            .expect("Failed to load preset on a different thread");
    }
}

impl ClapPlugin for Bells {
    const CLAP_ID: &'static str = "com.zmann.bells";
    const CLAP_DESCRIPTION: Option<&'static str> = None;
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = Some(Self::URL);

    // Don't forget to change these features
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::Sampler, ClapFeature::Instrument];
}

impl Vst3Plugin for Bells {
    const VST3_CLASS_ID: [u8; 16] = *b"zmann.bells.....";

    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Sampler, Vst3SubCategory::Instrument];
}

nih_export_clap!(Bells);
nih_export_vst3!(Bells);
