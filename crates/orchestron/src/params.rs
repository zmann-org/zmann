use std::sync::{atomic::{AtomicBool, Ordering}, Arc};

use nih_plug::prelude::*;
use crate::presets::Presets;

#[derive(Params)]
pub struct OrchestronParams {
    #[id = "gain"]
    pub gain: FloatParam,
    #[id = "preset"]
    pub preset: EnumParam<Presets>,
    #[allow(dead_code)]
    pub preset_changed: Arc<AtomicBool>,
}

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

impl Default for OrchestronParams {
    fn default() -> Self {
        let (preset_changed, preset_callback) = create_callback(|_: Presets| {});
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
            preset: EnumParam::new("Preset", Presets::default()).with_callback(preset_callback),
            // .hide(),
            preset_changed,
        }
    }
}
