use fx::{
    delay_line::StereoDelay,
    freeverb::Freeverb,
    moorer_verb::MoorerReverb,
    DEFAULT_SAMPLE_RATE,
};
use include_dir::{ include_dir, Dir };
use instrument::{ binv4::{ Instrument, PlayingStyle }, buffer::Sample };
use nih_plug::prelude::*;
use presets::Presets;
use std::sync::{ atomic::{ AtomicBool, Ordering }, Arc };
mod presets;

static ASSETS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../../samples/Toybox_c1200/");

const MAX_DELAY_TIME_SECONDS: f32 = 5.0;
const PARAMETER_MINIMUM: f32 = 0.01;

#[derive(Enum, Debug, PartialEq, Eq)]
pub enum ReverbType {
    #[id = "freeverb"]
    #[name = "Freeverb"]
    Freeverb,
    #[id = "moorer"]
    #[name = "Moorer"]
    Moorer,
}
struct ToyboxC {
    params: Arc<ToyboxCParams>,
    pub buffer: Vec<Sample>,
    instrument: Instrument,
    freeverb: Freeverb,
    moorer_reverb: MoorerReverb,
    chorus: StereoDelay,
}

#[derive(Params)]
struct ToyboxCParams {
    #[id = "input-gain"]
    pub reverb_gain: FloatParam,

    #[id = "output-gain"]
    pub output_gain: FloatParam,

    #[id = "reverb-dry-wet"]
    pub reverb_dry_wet_ratio: FloatParam,

    #[id = "reverb-room-size"]
    pub reverb_room_size: FloatParam,

    #[id = "reverb-dampening"]
    pub reverb_damping: FloatParam,

    #[id = "reverb-frozen"]
    pub reverb_frozen: BoolParam,

    #[id = "reverb-type"]
    pub reverb_type: EnumParam<ReverbType>,

    #[id = "reverb-width"]
    pub reverb_width: FloatParam,

    #[id = "chorus"]
    pub chorus: BoolParam,

    #[id = "chorus-rate"]
    pub chorus_rate: FloatParam,

    #[id = "chorus-lfo-amount"]
    pub chorus_lfo_amount: FloatParam,

    #[id = "chorus-depth"]
    pub chorus_depth: FloatParam,

    #[id = "chorus-width"]
    pub chorus_width: FloatParam,

    #[id = "chorus-feedback"]
    pub chorus_feedback: FloatParam,

    #[id = "preset"]
    pub preset: EnumParam<Presets>,
    preset_changed: Arc<AtomicBool>,
}

impl Default for ToyboxC {
    fn default() -> Self {
        Self {
            params: Arc::new(ToyboxCParams::default()),
            freeverb: Freeverb::new(DEFAULT_SAMPLE_RATE),
            moorer_reverb: MoorerReverb::new(DEFAULT_SAMPLE_RATE),
            chorus: StereoDelay::new(MAX_DELAY_TIME_SECONDS, DEFAULT_SAMPLE_RATE),
            buffer: vec![],
            instrument: Instrument::empty(),
        }
    }
}

impl Default for ToyboxCParams {
    fn default() -> Self {
        let preset_changed = Arc::new(AtomicBool::new(false));
        let preset_changed_mem = preset_changed.clone();
        let preset_callback = Arc::new(move |_: Presets| {
            preset_changed_mem.store(true, Ordering::Relaxed);
        });
        Self {
            reverb_gain: FloatParam::new("Reverb Gain", util::db_to_gain(0.0), FloatRange::Skewed {
                min: util::db_to_gain(-30.0),
                max: util::db_to_gain(30.0),
                factor: FloatRange::gain_skew_factor(-30.0, 30.0),
            })
                .with_smoother(SmoothingStyle::Logarithmic(50.0))
                .with_unit(" dB")
                .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
                .with_string_to_value(formatters::s2v_f32_gain_to_db()),
            output_gain: FloatParam::new("Output gain", util::db_to_gain(0.0), FloatRange::Skewed {
                min: util::db_to_gain(-30.0),
                max: util::db_to_gain(30.0),
                factor: FloatRange::gain_skew_factor(-30.0, 30.0),
            })
                .with_smoother(SmoothingStyle::Logarithmic(50.0))
                .with_unit(" dB")
                .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
                .with_string_to_value(formatters::s2v_f32_gain_to_db()),
            reverb_dry_wet_ratio: FloatParam::new("Reverb Dry/wet", 0.5, FloatRange::Linear {
                min: 0.0,
                max: 1.0,
            })
                .with_smoother(SmoothingStyle::Linear(50.0))
                .with_value_to_string(formatters::v2s_f32_rounded(2)),
            reverb_room_size: FloatParam::new("Reverb Room size", 0.5, FloatRange::Linear {
                min: 0.0,
                max: 1.0,
            })
                .with_smoother(SmoothingStyle::Linear(50.0))
                .with_value_to_string(formatters::v2s_f32_rounded(2)),
            reverb_damping: FloatParam::new("Reverb Dampening", 0.5, FloatRange::Linear {
                min: 0.0,
                max: 1.0,
            })
                .with_smoother(SmoothingStyle::Linear(50.0))
                .with_value_to_string(formatters::v2s_f32_rounded(2)),
            reverb_frozen: BoolParam::new("Reverb Frozen", false),
            reverb_type: EnumParam::new("Reverb Type", ReverbType::Freeverb),
            reverb_width: FloatParam::new("Reverb Width", 0.5, FloatRange::Linear {
                min: 0.0,
                max: 1.0,
            })
                .with_smoother(SmoothingStyle::Linear(50.0))
                .with_value_to_string(formatters::v2s_f32_rounded(2)),
            chorus: BoolParam::new("Chorus", false),
            chorus_rate: FloatParam::new("Chorus Rate", 0.1, FloatRange::Skewed {
                min: 0.001,
                max: 3.0,
                factor: FloatRange::skew_factor(-2.0),
            })
                .with_smoother(SmoothingStyle::Logarithmic(50.0))
                .with_unit(" Hz")
                .with_value_to_string(formatters::v2s_f32_rounded(2)),
            chorus_lfo_amount: FloatParam::new("Chorus LFO Amount", 0.02, FloatRange::Skewed {
                min: 0.001,
                max: 3.0,
                factor: FloatRange::skew_factor(-2.0),
            })
                .with_smoother(SmoothingStyle::Logarithmic(50.0))
                .with_unit(" freq. ratio")
                .with_value_to_string(formatters::v2s_f32_rounded(3)),
            chorus_depth: FloatParam::new("Chorus Depth", 0.5, FloatRange::Linear {
                min: PARAMETER_MINIMUM,
                max: 1.0,
            })
                .with_smoother(SmoothingStyle::Logarithmic(50.0))
                .with_value_to_string(formatters::v2s_f32_rounded(2)),
            chorus_width: FloatParam::new("Chorus Width", 0.5, FloatRange::Linear {
                min: PARAMETER_MINIMUM,
                max: 1.0,
            })
                .with_smoother(SmoothingStyle::Logarithmic(50.0))
                .with_value_to_string(formatters::v2s_f32_rounded(2)),
            chorus_feedback: FloatParam::new("Chorus Feedback", 0.5, FloatRange::Linear {
                min: PARAMETER_MINIMUM,
                max: 1.0,
            })
                .with_smoother(SmoothingStyle::Logarithmic(50.0))
                .with_value_to_string(formatters::v2s_f32_rounded(2)),
            preset: EnumParam::new("Preset", Presets::default()).with_callback(preset_callback), // .hide(),
            preset_changed,
        }
    }
}

impl ToyboxC {
    fn load_preset(&mut self, preset: Presets) {
        let before = std::time::Instant::now();
        if let Some(input_file) = ASSETS.get_file(format!("{}.binv4", preset.to_string())) {
            self.instrument = instrument::binv4::decode(input_file.contents().to_vec());
        }
        nih_log!(
            "[Toybox C1200] load_preset: {:.2?} - {:?} - {:?}",
            before.elapsed(),
            preset,
            self.params.preset.value()
        );
    }
    fn update_reverbs(&mut self) {
        let room_size_smoothed = &self.params.reverb_room_size.smoothed;
        let damping_smoothed = &self.params.reverb_damping.smoothed;
        let width_smoothed = &self.params.reverb_width.smoothed;

        // Update reverbs while parameters smooth
        if room_size_smoothed.is_smoothing() {
            self.freeverb.set_room_size(room_size_smoothed.next());
            self.moorer_reverb.set_room_size(room_size_smoothed.next());
        }
        if damping_smoothed.is_smoothing() {
            self.freeverb.set_damping(damping_smoothed.next());
            self.moorer_reverb.set_damping(damping_smoothed.next());
        }
        if width_smoothed.is_smoothing() {
            self.freeverb.set_width(width_smoothed.next());
            self.moorer_reverb.set_width(width_smoothed.next());
        }

        // Check if we should freeze the reverb
        let frozen = self.params.reverb_frozen.value();
        self.freeverb.set_frozen(frozen);
        self.moorer_reverb.set_frozen(frozen);
    }
}

impl Plugin for ToyboxC {
    const NAME: &'static str = "Toybox C1200";
    const VENDOR: &'static str = "ZMANN";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "info@example.com";

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
        self.chorus.resize_buffers(MAX_DELAY_TIME_SECONDS, _buffer_config.sample_rate as usize);
        self.freeverb.generate_filters(_buffer_config.sample_rate as usize);
        self.moorer_reverb.generate_filters(_buffer_config.sample_rate as usize);
        if self.instrument.notes.is_empty() {
            self.load_preset(self.params.preset.value());
        }
        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>
    ) -> ProcessStatus {
        let mut next_event = context.next_event();
        let preset_value_changed = self.params.preset_changed.clone();

        for (sample_id, mut channel_samples) in buffer.iter_samples().enumerate() {
            while let Some(event) = next_event {
                if event.timing() > (sample_id as u32) {
                    break;
                }
                match event {
                    NoteEvent::NoteOn { timing: _, voice_id: _, channel: _, note, velocity: _ } => {
                        if let Some(data) = self.instrument.notes.get(&note) {
                            nih_log!(
                                "[Toybox] NoteOn: {} - Buffer - {:?} Instrument - {:?}",
                                note,
                                std::thread::current().id(),
                                &self.params.preset.value()
                            );
                            self.buffer.push(Sample::new(data.to_vec(), note));
                        } else {
                            nih_log!(
                                "[Toybox] NO NOTE: {} - Buffer - {:?} Instrument - {:?}",
                                note,
                                std::thread::current().id(),
                                &self.params.preset.value()
                            );
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
                            match self.instrument.style {
                                PlayingStyle::WhilePressed => {
                                    self.buffer.remove(index);
                                    nih_log!(
                                        "[Toybox] NoteOff WhilePressed: {} - Buffer - {:?} Instrument - {:?}",
                                        note,
                                        std::thread::current().id(),
                                        &self.params.preset.value()
                                    );
                                }
                                _ => {
                                    nih_log!(
                                        "[Toybox] NoteOff: {} - Buffer - {:?} Instrument - {:?}",
                                        note,
                                        std::thread::current().id(),
                                        &self.params.preset.value()
                                    );
                                }
                            }
                        }
                    }
                    _ => (),
                }

                next_event = context.next_event();
            }

            let reverb_gain = self.params.reverb_gain.smoothed.next();
            let output_gain = self.params.output_gain.smoothed.next();

            for (i, sample) in channel_samples.iter_mut().enumerate() {
                let mut input = 0.0;

                for playing_sample in &mut self.buffer {
                    input += playing_sample.get_next_sample();
                }

                if self.params.chorus.value() {
                    let rate = self.params.chorus_rate.smoothed.next();
                    let vibrato_width = self.params.chorus_lfo_amount.smoothed.next();
                    let depth = self.params.chorus_depth.smoothed.next();
                    let width = self.params.chorus_width.smoothed.next() * 0.5;
                    let feedback = self.params.chorus_feedback.smoothed.next();

                    // this is neccicary to deinterleave the stereo signal
                    input = if i % 2 == 0 {
                        self.chorus.process_with_chorus(
                            (input, 0.0),
                            rate,
                            vibrato_width,
                            width,
                            depth,
                            feedback
                        ).0
                    } else {
                        self.chorus.process_with_chorus(
                            (0.0, input),
                            rate,
                            vibrato_width,
                            width,
                            depth,
                            feedback
                        ).1
                    };
                }

                let reverb = self.params.reverb_dry_wet_ratio.smoothed.next();
                if reverb > 0.0 {
                    self.update_reverbs();

                    // Process with reverb
                    input *= reverb_gain;
                    let frame_out = match self.params.reverb_type.value() {
                        ReverbType::Freeverb => {
                            if i % 2 == 0 {
                                self.freeverb.tick((input, 0.0)).0
                            } else {
                                self.freeverb.tick((0.0, input)).1
                            }
                        }
                        ReverbType::Moorer => {
                            if i % 2 == 0 {
                                self.moorer_reverb.tick((input, 0.0)).0
                            } else {
                                self.moorer_reverb.tick((0.0, input)).1
                            }
                        }
                    };

                    *sample = input * (1.0 - reverb) + frame_out * reverb;
                } else {
                    *sample = input;
                }

                *sample *= output_gain;

                self.buffer.retain(|e| !e.should_be_removed());
            }
        }

        if preset_value_changed.swap(false, Ordering::Relaxed) {
            nih_log!("[Toybox] preset_changed");
            self.load_preset(self.params.preset.value());
        }
        ProcessStatus::Normal
    }
}

impl Vst3Plugin for ToyboxC {
    const VST3_CLASS_ID: [u8; 16] = *b"zmann.c120012345";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Sampler,
        Vst3SubCategory::Instrument,
    ];
}

nih_export_vst3!(ToyboxC);
