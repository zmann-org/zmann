pub mod biquad;
pub mod dc_filter;
pub mod delay_line;
pub mod digital;
pub mod dynamics;
pub mod filters;
pub mod freeverb;
pub mod moorer_verb;
pub mod oversampling;
pub mod waveshapers;

// Constants for tape-modeled vibrato (wow & flutter)
pub const MAX_DELAY_TIME_SECONDS: f32 = 5.0;
pub const WOW_MAX_FREQUENCY_RATIO: f32 = 0.123;
pub const WOW_MAX_LFO_FREQUENCY: f32 = 0.816;
pub const FLUTTER_MAX_FREQUENCY_RATIO: f32 = 0.02;
pub const FLUTTER_MAX_LFO_FREQUENCY: f32 = 1.79;

// Constants for buffer instantiation
pub const DEFAULT_SAMPLE_RATE: usize = 48_000;
pub const ABLETON_LIVE_MAX_BUFFER_SIZE: usize = 2048;
