use nih_plug::prelude::*;
use presets::Presets;
use std::sync::{ atomic::Ordering, Arc };
use crate::params::OrchestronParams;
use instrument::microbuffer::Sample;
use crate::resampler::{ calc_hertz, resample };
use instrument::microbin::{ self, Instrument };
use rust_embed::RustEmbed;

mod params;
mod presets;
mod resampler;
mod editor;

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/../../samples/Orchestron/"]
struct Assets;

struct Orchestron {
    params: Arc<OrchestronParams>,
    pub buffer: Vec<Sample>,
    instrument: Instrument,
    sample_rate: f32,
}

impl Default for Orchestron {
    fn default() -> Self {
        Self {
            params: Arc::new(OrchestronParams::default()),
            buffer: vec![],
            instrument: Instrument::default(),
            sample_rate: 44100.0,
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

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> { 
        editor::create(self.params.clone())
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>
    ) -> bool {
        self.sample_rate = buffer_config.sample_rate;
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
        let preset_change = self.params.preset_change.clone();

        for (sample_id, channel_samples) in buffer.iter_samples().enumerate() {
            while let Some(event) = next_event {
                if event.timing() > (sample_id as u32) {
                    break;
                }
                match event {
                    NoteEvent::NoteOn { timing: _, voice_id: _, channel: _, note, mut velocity } => {
                        velocity = velocity * 1.5;
                        if note >= 12 && note <= 84 {
                            if note >= 12 && note <= 52 {
                                self.buffer.push(
                                    Sample::new(
                                        resample(
                                            &self.instrument.f0.to_vec(),
                                            44100.0,
                                            calc_hertz(self.sample_rate, 41 - (note as i32))
                                        ),
                                        note,
                                        velocity
                                    )
                                );
                            }

                            if note >= 53 && note <= 59 {
                                self.buffer.push(
                                    Sample::new(
                                        resample(
                                            &self.instrument.f1.to_vec(),
                                            44100.0,
                                            calc_hertz(self.sample_rate, 53 - (note as i32))
                                        ),
                                        note,
                                        velocity
                                    )
                                );
                            }

                            if note >= 60 && note <= 71 {
                                self.buffer.push(
                                    Sample::new(
                                        resample(
                                            &self.instrument.c2.to_vec(),
                                            44100.0,
                                            calc_hertz(self.sample_rate, 60 - (note as i32))
                                        ),
                                        note,
                                        velocity
                                    )
                                );
                            }

                            if note >= 72 && note <= 84 {
                                self.buffer.push(
                                    Sample::new(
                                        resample(
                                            &self.instrument.c3.to_vec(),
                                            44100.0,
                                            calc_hertz(self.sample_rate, 72 - (note as i32))
                                        ),
                                        note,
                                        velocity
                                    )
                                );
                            }
                        }
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

        if preset_change.load(Ordering::Relaxed) {
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
