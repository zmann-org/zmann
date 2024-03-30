use nih_plug::prelude::*;

#[derive(Params)]
pub struct OrchestronParams {
    #[id = "gain"]
    pub gain: FloatParam,
}

impl Default for OrchestronParams {
    fn default() -> Self {
        Self {
            gain: FloatParam::new("Gain", util::db_to_gain(0.0), FloatRange::Skewed {
                min: util::db_to_gain(-30.0),
                max: util::db_to_gain(30.0),
                factor: FloatRange::gain_skew_factor(-30.0, 30.0),
            })
                .with_smoother(SmoothingStyle::Logarithmic(50.0))
                .with_unit(" dB")
                .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
                .with_string_to_value(formatters::s2v_f32_gain_to_db()),
        }
    }
}