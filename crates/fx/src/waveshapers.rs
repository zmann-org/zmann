use std::f32::consts::{E, PI};

/// Processes an input sample through a static, saturating waveshaper.
/// Drive parameter increases the saturation.
///
/// Source: https://www.musicdsp.org/en/latest/Effects/46-waveshaper.html
pub fn get_saturator_output(drive: f32, input_sample: f32) -> f32 {
    let drive = drive.min(0.99);
    let k = 2.0 * drive / (1.0 - drive);
    let wet = ((1.0 + k) * input_sample) / (1.0 + k * (input_sample).abs());
    // Scale gain down to maintain similar loudness as input
    (1. - 0.3 * drive) * wet
}

/// Processes an input sample through a standard, static hard clipper, such
/// that the magnitude of the input sample will never surpass the threshold.
pub fn get_hard_clipper_output(threshold: f32, input_sample: f32) -> f32 {
    input_sample.clamp(-threshold, threshold)
}

/// Processes an input sample through a saturating, static hard clipper.
/// Drive parameter increases distortion and reduces threshold.
///
/// Desmos visualization of parameterization: https://www.desmos.com/calculator/ljssh5iqce
pub fn get_saturating_hard_clipper_output(drive: f32, input_sample: f32) -> f32 {
    let threshold = 1. - 0.5 * drive;
    let slope = 1. + 0.5 * drive;
    // Drive input into hard clipper for more distortion
    let x = input_sample * (1. + 4. * drive);
    if x.abs() < threshold {
        slope * x
    } else if slope * x > threshold {
        slope * threshold
    } else {
        -slope * threshold
    }
}

/// Processes an input sample through a fuzz inducing rectifier.
/// Drive parameter linearly changes waveshaper from a half-wave rectifier to a full-wave rectifier.
///
/// Desmos visualization of parameterization: https://www.desmos.com/calculator/hzttouljdp
pub fn get_fuzzy_rectifier_output(drive: f32, input_sample: f32) -> f32 {
    let x = input_sample;
    let output = if x >= 0. {
        input_sample
    } else {
        (1. - 2. * drive) * x
    };
    get_saturator_output(drive, output)
}

/// Processes an input sample through a rectifying curve modeled after a Shockley-Diode circuit.
/// Drive parameter changes the intensity of the curve.
///
/// Based off Chowdhury's Shockley Diode rectifier equation, modeled from William Shockley's work:
/// https://ccrma.stanford.edu/~jatin/papers/Complex_NLs.pdf
///
/// Desmos visualization of parameterization: https://www.desmos.com/calculator/wduyw6huen
pub fn get_shockley_diode_rectifier_output(drive: f32, input_sample: f32) -> f32 {
    let shockley_diode_output =
        (0.4 * drive + 0.1) * (E.powf((2. + 2. * drive) * input_sample) - 1.);
    // Run hard clipper in series to prevent clipping
    get_saturating_hard_clipper_output(drive, shockley_diode_output)
}

/// Processes an input sample through a dropout curve modeled after analog circuit response, where
/// lower input levels snap to zero.
/// Drive parameter changes the threshold of dropout.
///
/// Based off Chowdhury's Dropout equation:
/// https://ccrma.stanford.edu/~jatin/papers/Complex_NLs.pdf
///
/// Desmos visualization of parameterization: https://www.desmos.com/calculator/mv32dtqhwe
pub fn get_dropout_output(drive: f32, input_sample: f32) -> f32 {
    if drive == 0. {
        input_sample
    } else {
        let b = f32::sqrt(drive.powi(3) / 3.);
        let x = input_sample;
        let output = if x < -b {
            x + b - (b / drive).powi(3)
        } else if -b <= x && x <= b {
            (x / drive).powi(3)
        } else {
            x - b + (b / drive).powi(3)
        };
        get_saturating_hard_clipper_output(drive, output)
    }
}

fn cubic_waveshaper(x: f32) -> f32 {
    (0.75) * (x - x.powi(3) / 3.)
}

fn lower_waveshaper(x: f32, lower_skew_param: f32) -> f32 {
    let b = lower_skew_param;
    let b_recip = 1. / b;
    if x < -b_recip {
        -(0.5)
    } else if x > b_recip {
        0.5
    } else {
        cubic_waveshaper(lower_skew_param * x)
    }
}

/// Processes an input sample through an asymmetrical, "double soft clipper" waveshaper algorithm.
/// The drive parameter changes the upper limit of positive inputs and the skew of negative inputs.
///
/// Based off Chowdhury's double soft clipper:
/// https://ccrma.stanford.edu/~jatin/papers/Complex_NLs.pdf
/// Desmos visualization of parameterization: https://www.desmos.com/calculator/kngozoijks
pub fn get_double_soft_clipper_output(drive: f32, input_sample: f32) -> f32 {
    let x = input_sample;
    let upper_limit_param = 1. - 0.4 * drive;
    let lower_skew_param = 2. * drive + 1.;
    if -1. <= x && x <= 0. {
        let output = lower_waveshaper(2. * x + 1., lower_skew_param) - 0.5;
        get_saturator_output(drive, output)
    } else if 0. < x && x <= 1. {
        // Drive input value
        let x = x * 1.5;
        let output = upper_limit_param * (cubic_waveshaper(2. * x - 1.) + 0.5);
        get_saturator_output(drive, output)
    } else if x < -1. {
        -1.
    } else {
        1.
    }
}

/// Processes an input sample through a sinusoidal wavefolder.
/// The drive parameter increases the frequency of the sine curve, causing more distortion.
///
/// Desmos: https://www.desmos.com/calculator/zwffvndj7j
pub fn get_wavefolder_output(drive: f32, input_sample: f32) -> f32 {
    let k = 1. + (drive * 3.);
    let wet = (2. * PI * k * input_sample).sin();

    // Apply dry/wet based on drive to control volume
    let wet = (1. - drive) * input_sample + (drive) * wet;

    // Reduce gain as drive increases
    (1. - 0.3 * drive) * wet
}

// TODO: write more tests
#[cfg(test)]
mod tests {
    use approx::relative_eq;

    use super::*;

    #[test]
    fn shockley_diode_output_never_clips() {
        let drive = 1.0;
        for n in -100..100 {
            let n = n as f32 / 100.0;
            assert!(get_shockley_diode_rectifier_output(drive, n).abs() <= 1.);
        }
    }

    #[test]
    fn waveshapers_return_correct_dc_offset() {
        let num_drive_tests = 100;
        for test_num in 0..num_drive_tests {
            let drive = test_num as f32 / num_drive_tests as f32;
            // Use approx to avoid errors from floating point arithmetic
            assert!(relative_eq!(get_saturator_output(drive, 0.), 0.));
            assert!(relative_eq!(
                get_saturating_hard_clipper_output(drive, 0.),
                0.
            ));
            assert!(relative_eq!(get_hard_clipper_output(drive, 0.), 0.));
            assert!(relative_eq!(get_fuzzy_rectifier_output(drive, 0.), 0.));
            assert!(relative_eq!(
                get_shockley_diode_rectifier_output(drive, 0.),
                0.
            ));
            assert!(relative_eq!(get_dropout_output(drive, 0.), 0.));
            assert!(relative_eq!(get_double_soft_clipper_output(drive, 0.), 0.));
            assert!(relative_eq!(get_wavefolder_output(drive, 0.), 0.));
        }
    }

    #[test]
    fn hard_clip_clamps_correctly() {
        let threshold = 1.2;
        let max_range = 2.0;
        let num_values = 500;
        for i in 0..num_values {
            let input = (i as f32 / num_values as f32) * (max_range * 2.) - max_range;
            let output = get_hard_clipper_output(threshold, input);
            assert!(output.abs() <= threshold);
        }
    }
}
