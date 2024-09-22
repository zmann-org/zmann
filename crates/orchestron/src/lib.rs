#![allow(non_snake_case, non_upper_case_globals)]
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use common::buffer::Sample;
use common::resampler::{calc_hertz, resample};
use instrument::Instrument;
use nih_plug::prelude::*;
use presets::Presets;

pub mod instrument;
mod presets;

struct Orchestron {
    params: Arc<OrchestronParams>,
    pub buffer: Vec<Sample>,
    instrument: Instrument,
    sample_rate: f32,
}

#[derive(Params)]
struct OrchestronParams {
    #[id = "gain"]
    pub gain: FloatParam,
    #[id = "preset"]
    pub preset: EnumParam<Presets>,
    pub preset_change: Arc<AtomicBool>,
}

impl Default for Orchestron {
    fn default() -> Self {
        Self {
            params: Arc::new(OrchestronParams::default()),
            buffer: Vec::new(),
            instrument: Instrument::default(),
            sample_rate: 44100.0,
        }
    }
}

fn create_callback<T: 'static + Send + Sync>(
    f: impl Fn(T) + 'static + Send + Sync,
) -> (Arc<AtomicBool>, Arc<dyn Fn(T) + Send + Sync>) {
    let flag = Arc::new(AtomicBool::new(false));
    let flag_clone = Arc::clone(&flag);
    let callback = Arc::new(move |value: T| {
        f(value);
        flag_clone.store(true, Ordering::Relaxed);
    });
    (flag, callback)
}

impl Default for OrchestronParams {
    fn default() -> Self {
        let (preset_change, preset_callback) = create_callback(|_: Presets| {});
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
            preset: EnumParam::new("Preset", Presets::default()).with_callback(preset_callback),
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
        self.buffer.clear();
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let mut next_event = context.next_event();
        let preset_change = self.params.preset_change.clone();

        for (sample_id, channel_samples) in buffer.iter_samples().enumerate() {
            while let Some(event) = next_event {
                if event.timing() > (sample_id as u32) {
                    break;
                }
                match event {
                    NoteEvent::NoteOn {
                        timing: _,
                        voice_id: _,
                        channel: _,
                        note,
                        velocity,
                    } => {
                        self.buffer.push(Sample::new(
                            resample(
                                &self.instrument.sample.to_vec(),
                                44100.0,
                                calc_hertz(self.sample_rate, 53 - (note as i32)),
                            ),
                            note,
                            velocity,
                        ));
                    }
                    NoteEvent::NoteOff {
                        timing: _,
                        voice_id: _,
                        channel: _,
                        note,
                        velocity: _,
                    } => {
                        if let Some(index) = self.buffer.iter().position(|x| x.get_note_bool(note))
                        {
                            self.buffer.remove(index);
                        }
                    }
                    _ => (),
                }

                next_event = context.next_event();
            }

            let gain = self.params.gain.smoothed.next();

            for sample in channel_samples {
                for playing_sample in &mut self.buffer {
                    *sample += playing_sample.get_next_sample() * playing_sample.get_velocity();
                }

                *sample *= gain;

                self.buffer.retain(|e| !e.should_be_removed());
            }
        }

        if preset_change.load(Ordering::Relaxed)
            && self.instrument.name != self.params.preset.value().to_string()
        {
            self.load_preset(self.params.preset.value());
        }
        ProcessStatus::Normal
    }
}

impl Orchestron {
    pub fn load_preset(&mut self, preset: Presets) {
        self.buffer.clear();
        let instrument_data = preset.content().to_vec();
        let instrument = std::thread::spawn(move || {
            Instrument::decode(instrument_data)
        }).join().expect("Failed to load preset on a different thread");
        self.instrument = instrument;
    }
}

impl ClapPlugin for Orchestron {
    const CLAP_ID: &'static str = "com.zmann.orchestron";
    const CLAP_DESCRIPTION: Option<&'static str> = None;
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = Some(Self::URL);

    // Don't forget to change these features
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::Sampler, ClapFeature::Instrument];
}

impl Vst3Plugin for Orchestron {
    const VST3_CLASS_ID: [u8; 16] = *b"zmann.orchestron";

    // And also don't forget to change these categories
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Sampler, Vst3SubCategory::Instrument];
}

nih_export_clap!(Orchestron);
nih_export_vst3!(Orchestron);
