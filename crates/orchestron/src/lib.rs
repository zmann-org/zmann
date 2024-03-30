use nih_plug::prelude::*;
use presets::Presets;
use std::sync::{atomic::Ordering, Arc};
use crate::params::OrchestronParams;
use instrument::microbuffer::Sample;
use crate::resampler::{ calc_hertz, resample };
use instrument::microbin::{ self, Instrument };
use rust_embed::RustEmbed;

mod params;
mod presets;
mod resources;
mod resampler;

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/../../samples/Orchestron/"]
struct Assets;

struct Orchestron {
    params: Arc<OrchestronParams>,
    pub buffer: Vec<Sample>,
    instrument: Instrument,
}

impl Default for Orchestron {
    fn default() -> Self {
        Self {
            params: Arc::new(OrchestronParams::default()),
            buffer: vec![],
            instrument: Instrument::default(),
        }
    }
}

impl Plugin for Orchestron {
    const NAME: &'static str = "Orchestron";
    const VENDOR: &'static str = "ZMANN";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "info@zmann.org";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),

            aux_input_ports: &[],
            aux_output_ports: &[],

            names: PortNames::const_default(),
        },
    ];

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
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>
    ) -> bool {
        if self.instrument.f0.is_empty() {
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
        context: &mut impl ProcessContext<Self>
    ) -> ProcessStatus {
        let mut next_event = context.next_event();
        let preset_value_changed = self.params.preset_changed.clone();

        for (sample_id, channel_samples) in buffer.iter_samples().enumerate() {
            while let Some(event) = next_event {
                if event.timing() > (sample_id as u32) {
                    break;
                }
                match event {
                    NoteEvent::NoteOn { timing: _, voice_id: _, channel: _, note, velocity } => {
                        self.buffer.push(
                            Sample::new(
                                resample(
                                    &self.instrument.c2.to_vec(),
                                    44100,
                                    calc_hertz(44100.0, 60 - (note as i32)) as u32
                                ),
                                note,
                                1.0 // velocity * 1.5
                            )
                        );
                    }
                    NoteEvent::NoteOff {
                        timing: _,
                        voice_id: _,
                        channel: _,
                        note,
                        velocity: _,
                    } => {
                        if let Some(index) = self.buffer.iter().position(|x| x.get_note_bool(note)) {
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

        if preset_value_changed.swap(false, Ordering::Relaxed) {
            if self.instrument.name != self.params.preset.value().to_string() {
                self.load_preset(self.params.preset.value());
            }
        }

        ProcessStatus::Normal
    }
}

impl Orchestron {
    pub fn load_preset(&mut self, preset: Presets) {
        if
            let Some(input_file) = <Assets as rust_embed::RustEmbed>::get(
                &format!("{}.microbin", preset.to_string())
            )
        {
            self.instrument = microbin::decode(input_file.data.to_vec());
        }
    }
}

impl Vst3Plugin for Orchestron {
    const VST3_CLASS_ID: [u8; 16] = *b"zmann.orchestron";

    // And also don't forget to change these categories
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Sampler,
        Vst3SubCategory::Instrument,
    ];
}

nih_export_vst3!(Orchestron);
