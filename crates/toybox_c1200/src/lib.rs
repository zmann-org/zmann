use fx::{freeverb::Freeverb, moorer_verb::MoorerReverb, DEFAULT_SAMPLE_RATE};
use include_dir::{include_dir, Dir};
use instrument::{
    binv3::{Instrument, PlayingStyle},
    buffer::Sample,
};
use nih_plug::prelude::*;
use presets::Presets;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
mod presets;

static ASSETS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../../samples/Toybox_c1200/");

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
}

#[derive(Params)]
struct ToyboxCParams {
    #[id = "input-gain"]
    pub input_gain: FloatParam,

    #[id = "output-gain"]
    pub output_gain: FloatParam,

    #[id = "dry-wet"]
    pub dry_wet_ratio: FloatParam,

    #[id = "room-size"]
    pub room_size: FloatParam,

    #[id = "dampening"]
    pub damping: FloatParam,

    #[id = "frozen"]
    pub frozen: BoolParam,

    #[id = "reverb-type"]
    pub reverb_type: EnumParam<ReverbType>,

    #[id = "width"]
    pub width: FloatParam,
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
            input_gain: FloatParam::new(
                "Input gain",
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

            output_gain: FloatParam::new(
                "Output gain",
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

            dry_wet_ratio: FloatParam::new(
                "Dry/wet",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_value_to_string(formatters::v2s_f32_rounded(2)),

            room_size: FloatParam::new("Room size", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 })
                .with_smoother(SmoothingStyle::Linear(50.0))
                .with_value_to_string(formatters::v2s_f32_rounded(2)),

            damping: FloatParam::new("Dampening", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 })
                .with_smoother(SmoothingStyle::Linear(50.0))
                .with_value_to_string(formatters::v2s_f32_rounded(2)),

            frozen: BoolParam::new("Frozen", false),

            reverb_type: EnumParam::new("Type", ReverbType::Freeverb),

            width: FloatParam::new("Width", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 })
                .with_smoother(SmoothingStyle::Linear(50.0))
                .with_value_to_string(formatters::v2s_f32_rounded(2)),
            preset: EnumParam::new("Preset", Presets::default()).with_callback(preset_callback), // .hide(),
            preset_changed,
        }
    }
}

impl ToyboxC {
    fn load_preset(&mut self, preset: Presets) {
        nih_log!("[Toybox C1200] load_preset: {:?}", preset);
        if let Some(input_file) = ASSETS.get_file(format!("{}.binv3", preset.to_string())) {
            self.instrument = instrument::binv3::decode(input_file.contents().to_vec());
            nih_log!("[Toybox C1200] load_preset done: {:?}", preset);
        }
    }
    fn update_reverbs(&mut self) {
        let room_size_smoothed = &self.params.room_size.smoothed;
        let damping_smoothed = &self.params.damping.smoothed;
        let width_smoothed = &self.params.width.smoothed;

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
        let frozen = self.params.frozen.value();
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
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        self.load_preset(self.params.preset.value());
        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let mut next_event = context.next_event();
        let preset_value_changed = self.params.preset_changed.clone();

        for (sample_id, mut channel_samples) in buffer.iter_samples().enumerate() {
            while let Some(event) = next_event {
                if event.timing() > sample_id as u32 {
                    break;
                }
                match event {
                    NoteEvent::NoteOn {
                        timing: _,
                        voice_id: _,
                        channel: _,
                        note,
                        velocity: _,
                    } => {
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
                        if let Some(index) = self.buffer.iter().position(|x| x.get_note_bool(note))
                        {
                            if self.instrument.style == PlayingStyle::WhilePressed {
                                self.buffer.remove(index);
                                nih_log!(
                                        "[Toybox] NoteOff WhilePressed: {} - Buffer - {:?} Instrument - {:?}",
                                        note,
                                        std::thread::current().id(),
                                        &self.params.preset.value()
                                    );
                            } else {
                                nih_log!(
                                    "[Toybox] NoteOff: {} - Buffer - {:?} Instrument - {:?}",
                                    note,
                                    std::thread::current().id(),
                                    &self.params.preset.value()
                                );
                            }
                        }
                    }
                    _ => (),
                }

                next_event = context.next_event();
            }

            let output = self.params.output_gain.smoothed.next();

            for (i, sample) in channel_samples.iter_mut().enumerate() {
                let mut input = 0.0;
                for playing_sample in &mut self.buffer {
                    input += playing_sample.get_next_sample();
                }
            
                // Update reverbs based on parameters
                let room_size_smoothed = &self.params.room_size.smoothed;
                let damping_smoothed = &self.params.damping.smoothed;
                let width_smoothed = &self.params.width.smoothed;
            
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
                let frozen = self.params.frozen.value();
                self.freeverb.set_frozen(frozen);
                self.moorer_reverb.set_frozen(frozen);
            
                // Get input/output gain
                let input_gain = self.params.input_gain.smoothed.next();
                let output_gain = self.params.output_gain.smoothed.next();
            
                // Process with reverb
                input *= input_gain;
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
            
                // Apply dry/wet, then output
                let dry_wet_ratio = self.params.dry_wet_ratio.smoothed.next();
                *sample = input * (1. - dry_wet_ratio) + frame_out * dry_wet_ratio;
                *sample *= output_gain;
            
                *sample *= output;
            
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
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Sampler, Vst3SubCategory::Instrument];
}

nih_export_vst3!(ToyboxC);
