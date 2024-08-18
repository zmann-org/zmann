use dasp::{interpolate::linear::Linear, signal, Signal};

pub fn resample(
    data: &[f32],
    sample_rate0: f32,
    sample_rate: f32,
) -> Vec<f32> {
    let n = data.len() / (2 as usize);
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

pub fn calc_hertz(hz: f32, difference: i32) -> f32 {
    hz * f32::powf(2.0, (difference as f32) / 12.0)
}