use dasp::{interpolate::linear::Linear, signal, Signal};

/// Resamples the given audio data from one sample rate to another.
///
/// # Arguments
///
/// * `data` - A slice of f32 samples representing the audio data.
/// * `sample_rate0` - The original sample rate of the audio data.
/// * `sample_rate` - The target sample rate to resample the audio data to.
///
/// # Returns
///
/// A vector of f32 samples representing the resampled audio data.
///
/// # Panics
///
/// This function will panic if the input data length is not a multiple of 2.
pub fn resample(data: &[f32], sample_rate0: f32, sample_rate: f32) -> Vec<f32> {
    let n = data.len() / (2_usize);
    let n = n * sample_rate as usize / sample_rate0 as usize;
    let mut source = signal::from_interleaved_samples_iter::<_, [_; 2]>(data.iter().cloned());
    let a = source.next();
    let b = source.next();
    let interp = Linear::new(a, b);
    let mut data = Vec::with_capacity(n << 1);
    for x in source
        .from_hz_to_hz(interp, sample_rate0 as _, sample_rate as _)
        .take(n)
    {
        data.push(x[0]);
        data.push(x[1]);
    }
    data
}

/// Calculates the frequency in Hertz after applying a pitch shift.
///
/// # Arguments
///
/// * `hz` - The original frequency in Hertz.
/// * `difference` - The pitch shift in semitones.
///
/// # Returns
///
/// The frequency in Hertz after applying the pitch shift.
pub fn calc_hertz(hz: f32, difference: i32) -> f32 {
    hz * f32::powf(2.0, (difference as f32) / 12.0)
}
