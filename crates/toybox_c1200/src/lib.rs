use nih_plug::prelude::*;
use std::sync::Arc;
mod presets;
struct ToyboxC {
    params: Arc<ToyboxCParams>,
    pub buffer: Vec<instrument::buffer::Sample>,
    instrument: instrument::binv3::Instrument,
}

#[derive(Params)]
struct ToyboxCParams {
    #[id = "output"]
    pub output: FloatParam,
}

impl Default for ToyboxC {
    fn default() -> Self {
        Self {
            params: Arc::new(ToyboxCParams::default()),
            buffer: vec![],
            instrument: instrument::binv3::Instrument::new(),
        }
    }
}

impl Default for ToyboxCParams {
    fn default() -> Self {
        Self {
            output: FloatParam::new(
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
        }
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
        // Resize buffers and perform other potentially expensive initialization operations here.
        // The `reset()` function is always called right after this function. You can remove this
        // function if you do not need it.
        true
    }
    
    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        for channel_samples in buffer.iter_samples() {
            let output = self.params.output.smoothed.next();

            for sample in channel_samples {
                *sample *= output;
            }
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
