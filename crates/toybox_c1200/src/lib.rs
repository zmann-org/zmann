use fx::{
    biquad::{ BiquadFilterType, StereoBiquadFilter },
    delay_line::StereoDelay,
    freeverb::Freeverb,
    moorer_verb::MoorerReverb,
    DEFAULT_SAMPLE_RATE,
    FLUTTER_MAX_FREQUENCY_RATIO,
    FLUTTER_MAX_LFO_FREQUENCY,
    WOW_MAX_FREQUENCY_RATIO,
    WOW_MAX_LFO_FREQUENCY,
};
use instrument::{ binv5::{ Instrument, PlayingStyle }, buffer::Sample };
use nih_plug::prelude::*;
use nih_plug_webview::{ http::{ header::CONTENT_TYPE, Response }, HTMLSource, WebViewEditor };
use presets::Presets;
use rust_embed::RustEmbed;
use serde::Deserialize;
use serde_json::json;
use std::sync::{ atomic::{ AtomicBool, Ordering }, Arc };

mod presets;

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/../../packages/toybox_c1200_ui/dist/"]
struct WebAssets;
#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/../../samples/Toybox_c1200/"]
struct Assets;

const MAX_DELAY_TIME_SECONDS: f32 = 5.0;
const PARAMETER_MINIMUM: f32 = 0.01;

fn create_callback<T: 'static + Send + Sync>(
    f: impl Fn(T) + 'static + Send + Sync
) -> (Arc<AtomicBool>, Arc<dyn Fn(T) + Send + Sync>) {
    let flag = Arc::new(AtomicBool::new(false));
    let flag_clone = Arc::clone(&flag);
    let callback = Arc::new(move |value: T| {
        f(value);
        flag_clone.store(true, Ordering::Relaxed);
    });
    (flag, callback)
}

fn eq_type_to_param(filter_type: BiquadFilterTypeParam) -> BiquadFilterType {
    match filter_type {
        BiquadFilterTypeParam::LowPass => BiquadFilterType::LowPass,
        BiquadFilterTypeParam::HighPass => BiquadFilterType::HighPass,
        BiquadFilterTypeParam::BandPass => BiquadFilterType::BandPass,
        BiquadFilterTypeParam::Off => BiquadFilterType::LowPass,
    }
}

#[derive(Deserialize)]
#[serde(tag = "type")]
enum Action {
    Init,
    SetGain {
        value: f32,
    },
    SetPreset {
        preset: Presets,
    },
    SetReverbDryWet {
        value: f32,
    },
    SetReverbType {
        preset: ReverbType,
    },
    SetFilterType {
        preset: BiquadFilterTypeParam,
    },
    SetOutputGain {
        value: f32,
    },
}

#[derive(Enum, Debug, PartialEq, Eq, strum::Display, Deserialize)]
pub enum BiquadFilterTypeParam {
    LowPass,
    HighPass,
    BandPass,
    Off,
}

#[derive(Enum, Debug, PartialEq, Eq, strum::Display, Deserialize)]
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
    biquad: StereoBiquadFilter,
    freeverb: Freeverb,
    moorer_reverb: MoorerReverb,
    chorus: StereoDelay,
    wow: StereoDelay,
    flutter: StereoDelay,
}

#[derive(Params)]
struct ToyboxCParams {
    #[id = "output-gain"]
    pub output_gain: FloatParam,
    #[allow(dead_code)]
    output_gain_changed: Arc<AtomicBool>,

    #[id = "reverb-dry-wet"]
    pub reverb_dry_wet_ratio: FloatParam,
    #[allow(dead_code)]
    reverb_dry_wet_changed: Arc<AtomicBool>,

    #[id = "reverb-room-size"]
    pub reverb_room_size: FloatParam,
    #[allow(dead_code)]
    pub reverb_room_size_changed: Arc<AtomicBool>,

    #[id = "reverb-dampening"]
    pub reverb_damping: FloatParam,
    #[allow(dead_code)]
    pub reverb_damping_changed: Arc<AtomicBool>,

    #[id = "reverb-type"]
    pub reverb_type: EnumParam<ReverbType>,
    #[allow(dead_code)]
    reverb_type_changed: Arc<AtomicBool>,

    #[id = "reverb-width"]
    pub reverb_width: FloatParam,
    #[allow(dead_code)]
    pub reverb_width_changed: Arc<AtomicBool>,

    #[id = "chorus"]
    pub chorus: BoolParam,
    #[allow(dead_code)]
    pub chorus_changed: Arc<AtomicBool>,

    #[id = "chorus-rate"]
    pub chorus_rate: FloatParam,
    #[allow(dead_code)]
    pub chorus_rate_changed: Arc<AtomicBool>,

    #[id = "chorus-lfo-amount"]
    pub chorus_lfo_amount: FloatParam,
    #[allow(dead_code)]
    pub chorus_lfo_amount_changed: Arc<AtomicBool>,

    #[id = "chorus-depth"]
    pub chorus_depth: FloatParam,
    #[allow(dead_code)]
    pub chorus_depth_changed: Arc<AtomicBool>,

    #[id = "chorus-width"]
    pub chorus_width: FloatParam,
    #[allow(dead_code)]
    pub chorus_width_changed: Arc<AtomicBool>,

    #[id = "chorus-feedback"]
    pub chorus_feedback: FloatParam,
    #[allow(dead_code)]
    pub chorus_feedback_changed: Arc<AtomicBool>,

    #[id = "vibrato-wow"]
    pub vibrato_wow: FloatParam,
    #[allow(dead_code)]
    pub vibrato_wow_changed: Arc<AtomicBool>,

    #[id = "vibrato-flutter"]
    pub vibrato_flutter: FloatParam,
    #[allow(dead_code)]
    pub vibrato_flutter_changed: Arc<AtomicBool>,

    #[id = "filter-cutoff-frequency"]
    pub filter_cutoff_frequency: FloatParam,
    #[allow(dead_code)]
    pub filter_cutoff_frequency_changed: Arc<AtomicBool>,

    #[id = "filter-q"]
    pub filter_q: FloatParam,
    #[allow(dead_code)]
    pub filter_q_changed: Arc<AtomicBool>,

    #[id = "filter-type"]
    pub filter_type: EnumParam<BiquadFilterTypeParam>,
    #[allow(dead_code)]
    pub filter_type_changed: Arc<AtomicBool>,

    #[id = "preset"]
    pub preset: EnumParam<Presets>,
    #[allow(dead_code)]
    preset_changed: Arc<AtomicBool>,
}

impl Default for ToyboxC {
    fn default() -> Self {
        Self {
            params: Arc::new(ToyboxCParams::default()),
            freeverb: Freeverb::new(DEFAULT_SAMPLE_RATE),
            moorer_reverb: MoorerReverb::new(DEFAULT_SAMPLE_RATE),
            chorus: StereoDelay::new(MAX_DELAY_TIME_SECONDS, DEFAULT_SAMPLE_RATE),
            wow: StereoDelay::new(MAX_DELAY_TIME_SECONDS, DEFAULT_SAMPLE_RATE),
            flutter: StereoDelay::new(MAX_DELAY_TIME_SECONDS, DEFAULT_SAMPLE_RATE),
            biquad: StereoBiquadFilter::new(),
            buffer: vec![],
            instrument: Instrument::empty(),
        }
    }
}

impl Default for ToyboxCParams {
    fn default() -> Self {
        let (preset_changed, preset_callback) = create_callback(|_: Presets| {});
        let (reverb_dry_wet_changed, reverb_dry_wet_value_param_callback) = create_callback(
            |_: f32| {}
        );
        let (reverb_type_changed, reverb_type_changed_param_callback) = create_callback(
            |_: ReverbType| {}
        );
        let (reverb_room_size_changed, reverb_room_size_changed_param_callback) = create_callback(
            |_: f32| {}
        );
        let (reverb_damping_changed, reverb_damping_changed_param_callback) = create_callback(
            |_: f32| {}
        );
        let (reverb_width_changed, reverb_width_changed_param_callback) = create_callback(
            |_: f32| {}
        );
        let (chorus_changed, chorus_changed_param_callback) = create_callback(|_: bool| {});
        let (chorus_rate_changed, chorus_rate_changed_param_callback) = create_callback(
            |_: f32| {}
        );
        let (chorus_lfo_amount_changed, chorus_lfo_amount_changed_param_callback) = create_callback(
            |_: f32| {}
        );
        let (chorus_depth_changed, chorus_depth_changed_param_callback) = create_callback(
            |_: f32| {}
        );
        let (chorus_width_changed, chorus_width_changed_param_callback) = create_callback(
            |_: f32| {}
        );
        let (chorus_feedback_changed, chorus_feedback_changed_param_callback) = create_callback(
            |_: f32| {}
        );
        let (vibrato_wow_changed, vibrato_wow_changed_param_callback) = create_callback(
            |_: f32| {}
        );
        let (vibrato_flutter_changed, vibrato_flutter_changed_param_callback) = create_callback(
            |_: f32| {}
        );
        let (filter_cutoff_frequency_changed, filter_cutoff_frequency_changed_param_callback) =
            create_callback(|_: f32| {});
        let (filter_q_changed, filter_q_changed_param_callback) = create_callback(|_: f32| {});
        let (filter_type_changed, filter_type_changed_param_callback) = create_callback(
            |_: BiquadFilterTypeParam| {}
        );
        let (output_gain_changed, output_gain_changed_param_callback) = create_callback(
            |_: f32| {}
        );

        Self {
            output_gain: FloatParam::new("Output gain", util::db_to_gain(0.0), FloatRange::Skewed {
                min: util::db_to_gain(-30.0),
                max: util::db_to_gain(30.0),
                factor: FloatRange::gain_skew_factor(-30.0, 30.0),
            })
                .with_smoother(SmoothingStyle::Logarithmic(50.0))
                .with_unit(" dB")
                .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
                .with_string_to_value(formatters::s2v_f32_gain_to_db())
                .with_callback(output_gain_changed_param_callback),
            output_gain_changed,
            reverb_dry_wet_ratio: FloatParam::new("Reverb Dry/wet", 0.5, FloatRange::Linear {
                min: 0.0,
                max: 1.0,
            })
                .with_smoother(SmoothingStyle::Linear(50.0))
                .with_value_to_string(formatters::v2s_f32_rounded(2))
                .with_callback(reverb_dry_wet_value_param_callback),
            reverb_dry_wet_changed,
            reverb_room_size: FloatParam::new("Reverb Room size", 0.5, FloatRange::Linear {
                min: 0.0,
                max: 1.0,
            })
                .with_smoother(SmoothingStyle::Linear(50.0))
                .with_value_to_string(formatters::v2s_f32_rounded(2))
                .with_callback(reverb_room_size_changed_param_callback),
            reverb_room_size_changed,
            reverb_damping: FloatParam::new("Reverb Dampening", 0.5, FloatRange::Linear {
                min: 0.0,
                max: 1.0,
            })
                .with_smoother(SmoothingStyle::Linear(50.0))
                .with_value_to_string(formatters::v2s_f32_rounded(2))
                .with_callback(reverb_damping_changed_param_callback),
            reverb_damping_changed,
            reverb_type: EnumParam::new("Reverb Type", ReverbType::Freeverb).with_callback(
                reverb_type_changed_param_callback
            ),
            reverb_type_changed,
            reverb_width: FloatParam::new("Reverb Width", 0.5, FloatRange::Linear {
                min: 0.0,
                max: 1.0,
            })
                .with_smoother(SmoothingStyle::Linear(50.0))
                .with_value_to_string(formatters::v2s_f32_rounded(2))
                .with_callback(reverb_width_changed_param_callback),
            reverb_width_changed,
            chorus: BoolParam::new("Chorus", false).with_callback(chorus_changed_param_callback),
            chorus_changed,
            chorus_rate: FloatParam::new("Chorus Rate", 0.1, FloatRange::Skewed {
                min: 0.001,
                max: 3.0,
                factor: FloatRange::skew_factor(-2.0),
            })
                .with_smoother(SmoothingStyle::Logarithmic(50.0))
                .with_unit(" Hz")
                .with_value_to_string(formatters::v2s_f32_rounded(2))
                .with_callback(chorus_rate_changed_param_callback),
            chorus_rate_changed,
            chorus_lfo_amount: FloatParam::new("Chorus LFO Amount", 0.02, FloatRange::Skewed {
                min: 0.001,
                max: 3.0,
                factor: FloatRange::skew_factor(-2.0),
            })
                .with_smoother(SmoothingStyle::Logarithmic(50.0))
                .with_unit(" freq. ratio")
                .with_value_to_string(formatters::v2s_f32_rounded(3))
                .with_callback(chorus_lfo_amount_changed_param_callback),
            chorus_lfo_amount_changed,
            chorus_depth: FloatParam::new("Chorus Depth", 0.5, FloatRange::Linear {
                min: PARAMETER_MINIMUM,
                max: 1.0,
            })
                .with_smoother(SmoothingStyle::Logarithmic(50.0))
                .with_value_to_string(formatters::v2s_f32_rounded(2))
                .with_callback(chorus_depth_changed_param_callback),
            chorus_depth_changed,
            chorus_width: FloatParam::new("Chorus Width", 0.5, FloatRange::Linear {
                min: PARAMETER_MINIMUM,
                max: 1.0,
            })
                .with_smoother(SmoothingStyle::Logarithmic(50.0))
                .with_value_to_string(formatters::v2s_f32_rounded(2))
                .with_callback(chorus_width_changed_param_callback),
            chorus_width_changed,
            chorus_feedback: FloatParam::new("Chorus Feedback", 0.5, FloatRange::Linear {
                min: PARAMETER_MINIMUM,
                max: 1.0,
            })
                .with_smoother(SmoothingStyle::Logarithmic(50.0))
                .with_value_to_string(formatters::v2s_f32_rounded(2))
                .with_callback(chorus_feedback_changed_param_callback),
            chorus_feedback_changed,
            vibrato_wow: FloatParam::new("Vibrato Wow", 0.0, FloatRange::Skewed {
                min: PARAMETER_MINIMUM,
                max: 1.0,
                factor: FloatRange::skew_factor(-1.0),
            })
                .with_smoother(SmoothingStyle::Logarithmic(50.0))
                .with_value_to_string(formatters::v2s_f32_rounded(2))
                .with_callback(vibrato_wow_changed_param_callback),
            vibrato_wow_changed,
            vibrato_flutter: FloatParam::new("Vibrato Flutter", 0.0, FloatRange::Skewed {
                min: PARAMETER_MINIMUM,
                max: 1.0,
                factor: FloatRange::skew_factor(-1.0),
            })
                .with_smoother(SmoothingStyle::Logarithmic(50.0))
                .with_value_to_string(formatters::v2s_f32_rounded(2))
                .with_callback(vibrato_flutter_changed_param_callback),
            vibrato_flutter_changed,
            filter_cutoff_frequency: FloatParam::new("Filter Cutoff", 1_000.0, FloatRange::Skewed {
                min: 15.0,
                max: 22_000.0,
                factor: FloatRange::skew_factor(-2.2),
            })
                .with_smoother(SmoothingStyle::Logarithmic(20.0))
                .with_unit(" Hz")
                .with_value_to_string(formatters::v2s_f32_rounded(2))
                .with_callback(filter_cutoff_frequency_changed_param_callback),
            filter_cutoff_frequency_changed,
            filter_q: FloatParam::new("Filter Q", 0.7, FloatRange::Skewed {
                min: 0.1,
                max: 18.0,
                factor: FloatRange::skew_factor(-2.2),
            })
                .with_smoother(SmoothingStyle::Logarithmic(20.0))
                .with_value_to_string(formatters::v2s_f32_rounded(2))
                .with_callback(filter_q_changed_param_callback),
            filter_q_changed,
            filter_type: EnumParam::new("Filter Type", BiquadFilterTypeParam::Off).with_callback(
                filter_type_changed_param_callback
            ),
            filter_type_changed,
            preset: EnumParam::new("Preset", Presets::default())
                .with_callback(preset_callback)
                .hide(),
            preset_changed,
        }
    }
}

impl ToyboxC {
    fn load_preset(&mut self, preset: Presets) {
        let before = std::time::Instant::now();
        if
            let Some(input_file) = <Assets as rust_embed::RustEmbed>::get(
                &format!("{}.binv5", preset.to_string())
            )
        {
            self.instrument = instrument::binv5::decode(input_file.data.to_vec());
            nih_log!("[Toybox C1200] Loaded preset: {:?}", preset.to_string());
        }
        nih_log!(
            "[Toybox C1200] load_preset: {:.2?} - {:?} - {:?}",
            before.elapsed(),
            preset.to_string(),
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

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        let params = self.params.clone();
        let reverb_dry_wet_changed = self.params.reverb_dry_wet_changed.clone();
        let preset_value_changed = self.params.preset_changed.clone();
        let reverb_type_changed = self.params.reverb_type_changed.clone();
        let filter_type_changed = self.params.filter_type_changed.clone();
        let output_gain_changed = self.params.output_gain_changed.clone();

        let mut editor = WebViewEditor::new(
            HTMLSource::URL("zmann://localhost/index.html"),
            (800, 350)
        )
            .with_custom_protocol("zmann".into(), move |request| {
                let path = request.uri().path();
                let mimetype = if path.ends_with(".html") {
                    "text/html"
                } else if path.ends_with(".js") {
                    "text/javascript"
                } else if path.ends_with(".css") {
                    "text/css"
                } else if path.ends_with(".png") {
                    "image/png"
                } else {
                    "application/octet-stream" // falback, replace with mime_guess
                };

                match <WebAssets as rust_embed::RustEmbed>::get(path.trim_start_matches("/")) {
                    Some(content) => {
                        return Response::builder()
                            .header(CONTENT_TYPE, mimetype)
                            .header("Access-Control-Allow-Origin", "*")
                            .body(content.data.to_vec().into())
                            .map_err(Into::into);
                    }
                    None => {
                        return Response::builder()
                            .header(CONTENT_TYPE, "text/html")
                            .header("Access-Control-Allow-Origin", "*")
                            .body((b"not found" as &[u8]).into())
                            .map_err(Into::into);
                    }
                }
            })
            .with_background_color((40, 39, 41, 255))
            .with_developer_mode(true)
            .with_event_loop(move |ctx, setter, _window| {
                while let Ok(value) = ctx.next_event() {
                    if let Ok(action) = serde_json::from_value(value.clone()) {
                        match action {
                            Action::SetGain { value } => {
                                setter.begin_set_parameter(&params.output_gain);
                                setter.set_parameter_normalized(&params.output_gain, value);
                                setter.end_set_parameter(&params.output_gain);
                            }
                            Action::SetPreset { preset } => {
                                setter.begin_set_parameter(&params.preset);
                                setter.set_parameter(&params.preset, preset);
                                setter.end_set_parameter(&params.preset);
                            }
                            Action::SetReverbType { preset } => {
                                setter.begin_set_parameter(&params.reverb_type);
                                setter.set_parameter(&params.reverb_type, preset);
                                setter.end_set_parameter(&params.reverb_type);
                            }
                            Action::SetFilterType { preset } => {
                                setter.begin_set_parameter(&params.filter_type);
                                setter.set_parameter(&params.filter_type, preset);
                                setter.end_set_parameter(&params.filter_type);
                            }
                            Action::SetReverbDryWet { value } => {
                                setter.begin_set_parameter(&params.reverb_dry_wet_ratio);
                                setter.set_parameter_normalized(
                                    &params.reverb_dry_wet_ratio,
                                    value
                                );
                                setter.end_set_parameter(&params.reverb_dry_wet_ratio);
                            }
                            Action::SetOutputGain { value } => {
                                setter.begin_set_parameter(&params.output_gain);
                                setter.set_parameter_normalized(&params.output_gain, value);
                                setter.end_set_parameter(&params.output_gain);
                            }
                            Action::Init => {
                                let _ = ctx.send_json(
                                    json!({
                                        "type": "preset_change",
                                        "value": params.preset.value().to_string(),
                                    })
                                );
                                let _ = ctx.send_json(
                                    json!({
                                        "type": "reverb_dry_wet_change",
                                        "value": params.reverb_dry_wet_ratio.value().to_string(),
                                    })
                                );
                                let _ = ctx.send_json(
                                    json!({
                                        "type": "reverb_type_changed",
                                        "value": params.reverb_type.value().to_string(),
                                    })
                                );
                                let _ = ctx.send_json(
                                    json!({
                                        "type": "filter_type_changed",
                                        "value": params.filter_type.value().to_string(),
                                    })
                                );
                                let _ = ctx.send_json(
                                    json!({
                                        "type": "output_gain_changed",
                                        "value": params.output_gain.modulated_normalized_value().to_string(),
                                    })
                                );
                            }
                        }
                    }
                }

                if reverb_dry_wet_changed.swap(false, Ordering::Relaxed) {
                    let _ = ctx.send_json(
                        json!({
                            "type": "reverb_dry_wet_change",
                            "value": params.reverb_dry_wet_ratio.value().to_string(),
                        })
                    );
                }

                if reverb_type_changed.swap(false, Ordering::Relaxed) {
                    let _ = ctx.send_json(
                        json!({
                            "type": "reverb_type_changed",
                            "value": params.reverb_type.value().to_string(),
                        })
                    );
                }

                if preset_value_changed.swap(false, Ordering::Relaxed) {
                    let _ = ctx.send_json(
                        json!({
                            "type": "preset_change",
                            "value": params.preset.value().to_string(),
                            "text": params.preset.to_string()
                        })
                    );
                }

                if filter_type_changed.swap(false, Ordering::Relaxed) {
                    let _ = ctx.send_json(
                        json!({
                            "type": "filter_type_changed",
                            "value": params.filter_type.value().to_string(),
                        })
                    );
                }

                if output_gain_changed.swap(false, Ordering::Relaxed) {
                    let _ = ctx.send_json(
                        json!({
                            "type": "output_gain_changed",
                            "value": params.output_gain.modulated_normalized_value().to_string(),
                        })
                    );
                }
            });

            #[cfg(windows)]
            {
                editor = editor.with_caption_color(0x00292728);
            }
            
        Some(Box::new(editor))
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>
    ) -> bool {
        let before: std::time::Instant = std::time::Instant::now();
        self.freeverb.set_frozen(false);
        self.moorer_reverb.set_frozen(false);
        self.wow.resize_buffers(MAX_DELAY_TIME_SECONDS, _buffer_config.sample_rate as usize);
        self.flutter.resize_buffers(MAX_DELAY_TIME_SECONDS, _buffer_config.sample_rate as usize);
        self.chorus.resize_buffers(MAX_DELAY_TIME_SECONDS, _buffer_config.sample_rate as usize);
        self.freeverb.generate_filters(_buffer_config.sample_rate as usize);
        self.moorer_reverb.generate_filters(_buffer_config.sample_rate as usize);
        if self.instrument.notes.is_empty() {
            self.load_preset(self.params.preset.value());
        }
        nih_log!(
            "[Toybox C1200] initialize: {:.2?} - {:?}",
            before.elapsed(),
            self.params.preset.value()
        );
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
                            self.buffer.push(Sample::new(data.to_vec(), note));
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
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => (),
                }

                next_event = context.next_event();
            }

            let output_gain = self.params.output_gain.smoothed.next();

            for (i, sample) in channel_samples.iter_mut().enumerate() {
                let mut input = 0.0;

                for playing_sample in &mut self.buffer {
                    input += playing_sample.get_next_sample();
                }

                let wow = self.params.vibrato_wow.smoothed.next();
                let flutter = self.params.vibrato_flutter.smoothed.next();

                if self.params.filter_type.value() != BiquadFilterTypeParam::Off {
                    let filter_type = self.params.filter_type.value();
                    let frequency = self.params.filter_cutoff_frequency.smoothed.next();
                    let fc = frequency / (DEFAULT_SAMPLE_RATE as f32);
                    let q = self.params.filter_q.smoothed.next();

                    self.biquad.set_biquads(
                        eq_type_to_param(filter_type),
                        fc,
                        q,
                        util::gain_to_db(0.0)
                    );

                    if self.params.filter_cutoff_frequency.smoothed.is_smoothing() {
                        let cutoff_frequency_smoothed =
                            self.params.filter_cutoff_frequency.smoothed.next();
                        let fc = cutoff_frequency_smoothed / (DEFAULT_SAMPLE_RATE as f32);
                        self.biquad.set_fc(fc);
                    }
                    if self.params.filter_q.smoothed.is_smoothing() {
                        let q_smoothed = self.params.filter_q.smoothed.next();
                        self.biquad.set_q(q_smoothed);
                    }
                    input = if i % 2 == 0 {
                        self.biquad.process((input, 0.0)).0
                    } else {
                        self.biquad.process((0.0, input)).1
                    };
                }

                if wow > PARAMETER_MINIMUM {
                    input = if i % 2 == 0 {
                        self.wow.process_with_vibrato(
                            (input, 0.0),
                            WOW_MAX_LFO_FREQUENCY,
                            wow * WOW_MAX_FREQUENCY_RATIO,
                            0.05
                        ).0
                    } else {
                        self.wow.process_with_vibrato(
                            (0.0, input),
                            WOW_MAX_LFO_FREQUENCY,
                            wow * WOW_MAX_FREQUENCY_RATIO,
                            0.05
                        ).1
                    };
                }

                if flutter > PARAMETER_MINIMUM {
                    input = (
                        if i % 2 == 0 {
                            self.flutter.process_with_vibrato(
                                (input, 0.0),
                                FLUTTER_MAX_LFO_FREQUENCY,
                                flutter * FLUTTER_MAX_FREQUENCY_RATIO,
                                0.05
                            )
                        } else {
                            self.flutter.process_with_vibrato(
                                (0.0, input),
                                FLUTTER_MAX_LFO_FREQUENCY,
                                flutter * FLUTTER_MAX_FREQUENCY_RATIO,
                                0.05
                            )
                        }
                    ).1;
                }

                if self.params.chorus.value() {
                    let rate = self.params.chorus_rate.smoothed.next();
                    let vibrato_width = self.params.chorus_lfo_amount.smoothed.next();
                    let depth = self.params.chorus_depth.smoothed.next();
                    let width = self.params.chorus_width.smoothed.next() * 0.5;
                    let feedback = self.params.chorus_feedback.smoothed.next();

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
                if reverb > PARAMETER_MINIMUM {
                    self.update_reverbs();
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

        if preset_value_changed.load(Ordering::Relaxed) {
            if self.instrument.name != self.params.preset.value().to_string() {
                self.load_preset(self.params.preset.value());
            }
        }
        ProcessStatus::Normal
    }
}

impl Vst3Plugin for ToyboxC {
    const VST3_CLASS_ID: [u8; 16] = *b"zmann.c120012345"; // NOTE: change to standard ID following our naming convention
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Sampler,
        Vst3SubCategory::Instrument,
    ];
}

nih_export_vst3!(ToyboxC);
