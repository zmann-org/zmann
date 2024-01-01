use std::f32::consts::E;

const AVERAGE_FACTOR: f32 = 0.9999;

/// FIXME: Use with extreme caution and low volumes.
/// I probably implemented this wrong, because Juan Gil's JUCE version sounds fine but this does not.
/// 
/// A dynamic range processor capable of compression and expansion.
///
/// Code ported from Juan Gil's compressor-expander implementation, which is licensed under GNU:
// https://github.com/juandagilc/Audio-Effects/blob/master/Compressor-Expander/Source/PluginProcessor.cpp
pub struct DynamicRangeProcessor {
    sample_rate: usize,
    input_level: f32,
    yl_prev: f32,
    xg: f32,
    xl: f32,
    yg: f32,
    yl: f32,

    threshold: f32,
    ratio: f32,
    attack: f32,
    release: f32,
    is_expander: bool,
}

impl DynamicRangeProcessor {
    pub fn new(sample_rate: usize) -> DynamicRangeProcessor {
        DynamicRangeProcessor {
            sample_rate,
            input_level: 0.,
            yl_prev: 0.,
            xg: 0.0,
            xl: 0.0,
            yg: 0.0,
            yl: 0.0,
            threshold: 0.,
            ratio: 1.,
            attack: 0.,
            release: 0.,
            is_expander: false,
        }
    }

    ///
    /// Update the parameters of the dynamic range processor.
    ///
    /// # Arguments
    /// * `threshold` - the threshold at what level to enact dynamic range processing, in dBFS
    /// * `ratio` - the amount of attenuation after the input signal crosses the threshold
    /// * `attack` - the amount of time to reach full attenuation, in seconds
    /// * `release` - the amount of time to stop attenuation, in seconds
    /// * `is_expander` - when false, compress when input exceeds threshold; otherwise, expand when input falls below threshold
    ///
    pub fn set_parameters(
        &mut self,
        threshold: f32,
        ratio: f32,
        attack: f32,
        release: f32,
        is_expander: bool,
    ) {
        self.threshold = threshold;
        self.ratio = ratio;
        self.attack = attack;
        self.release = release;
        self.is_expander = is_expander;
    }

    pub fn set_sample_rate(&mut self, sample_rate: usize) {
        self.sample_rate = sample_rate;
    }

    fn calculate_alpha_time(&self, tau: f32) -> f32 {
        if tau == 0. {
            tau
        } else {
            E.recip().powf((self.sample_rate as f32).recip() / tau)
        }
    }

    ///
    /// Convert stereo (2-channel) buffer to mono
    ///
    pub fn mix_down_input(buffer: &Vec<(f32, f32)>) -> Vec<f32> {
        let mixed_down: Vec<f32> = buffer.into_iter().map(|x| (x.0 + x.1) / 2.).collect();
        mixed_down
    }

    /// Calculates control voltage to apply to input based on
    /// compressor's internal parameters.
    ///
    /// # Arguments
    /// * `input` - a single input sample
    /// * `makeup_gain` - the makeup gain to apply after compression
    ///
    pub fn calculate_control_voltage(&mut self, input: f32, makeup_gain: f32) -> f32 {
        // Get internal parameters
        let threshold = self.threshold;
        let alpha_attack = self.calculate_alpha_time(self.attack);
        let alpha_release = self.calculate_alpha_time(self.release);

        let input_squared = input.powf(2.);
        self.input_level = if self.is_expander {
            AVERAGE_FACTOR * self.input_level + (1. - AVERAGE_FACTOR) * input_squared
        } else {
            input_squared
        };

        self.xg = if self.input_level <= 0.000001 {
            -60.
        } else {
            10. * self.input_level.log10()
        };

        if self.is_expander {
            // Expand
            self.yg = if self.xg > threshold {
                self.xg
            } else {
                threshold + (self.xg - threshold) * self.ratio
            };

            self.xl = self.xg - self.yg;

            self.yl = if self.xl < self.yl_prev {
                alpha_attack * self.yl_prev + (1. - alpha_attack) * self.xl
            } else {
                alpha_release * self.yl_prev + (1. - alpha_release) * self.xl
            };
        } else {
            // Compress
            self.yg = if self.xg < threshold {
                self.xg
            } else {
                threshold + (self.xg - threshold) * self.ratio
            };

            self.xl = self.xg - self.yg;

            self.yl = if self.xl > self.yl_prev {
                alpha_attack * self.yl_prev + (1. - alpha_attack) * self.xl
            } else {
                alpha_release * self.yl_prev + (1. - alpha_release) * self.xl
            };
        }

        let control_voltage = 10.0_f32.powf((makeup_gain - self.yl) * 0.05);
        self.yl_prev = self.yl;

        control_voltage
    }

    /// Calculate control voltage signal for a stereo input buffer with static makeup gain
    pub fn calculate_cv_signal(&mut self, buffer: &Vec<(f32, f32)>, makeup_gain: f32) -> Vec<f32> {
        let mixed_down_input = DynamicRangeProcessor::mix_down_input(buffer);
        mixed_down_input
            .iter()
            .map(|x| self.calculate_control_voltage(*x, makeup_gain))
            .collect()
    }

    ///
    /// Processes input samples based on internal parameters.
    ///
    /// # Arguments
    /// * `input_frame` - a stereo frame of input
    /// * `makeup_gain` - the makeup gain to apply after processing, in dB
    ///
    pub fn process_input_frame(&mut self, input_frame: (f32, f32), makeup_gain: f32) -> (f32, f32) {
        // Get internal parameters
        let threshold = self.threshold;
        let alpha_attack = self.calculate_alpha_time(self.attack);
        let alpha_release = self.calculate_alpha_time(self.release);

        let input = (input_frame.0 + input_frame.1) * 0.5;

        let input_squared = input.powf(2.);
        self.input_level = if self.is_expander {
            AVERAGE_FACTOR * self.input_level + (1. - AVERAGE_FACTOR) * input_squared
        } else {
            input_squared
        };

        self.xg = if self.input_level <= 0.000001 {
            -60.
        } else {
            10. * self.input_level.log10()
        };

        if self.is_expander {
            // Compute gain below threshold (expansion)
            self.yg = if self.xg > threshold {
                self.xg
            } else {
                threshold + (self.xg - threshold) * self.ratio
            };

            self.xl = self.xg - self.yg;

            // Ballistics; apply attack or release
            self.yl = if self.xl < self.yl_prev {
                alpha_attack * self.yl_prev + (1. - alpha_attack) * self.xl
            } else {
                alpha_release * self.yl_prev + (1. - alpha_release) * self.xl
            };
        } else {
            // Compute gain above threshold (compression)
            self.yg = if self.xg < threshold {
                self.xg
            } else {
                threshold + (self.xg - threshold) * self.ratio
            };

            self.xl = self.xg - self.yg;

            // Ballistics; apply attack or release
            self.yl = if self.xl > self.yl_prev {
                alpha_attack * self.yl_prev + (1. - alpha_attack) * self.xl
            } else {
                alpha_release * self.yl_prev + (1. - alpha_release) * self.xl
            };
        }

        let control_voltage = 10.0_f32.powf((makeup_gain - self.yl) * 0.05);
        self.yl_prev = self.yl;

        (
            input_frame.0 * control_voltage,
            input_frame.1 * control_voltage,
        )
    }
}
