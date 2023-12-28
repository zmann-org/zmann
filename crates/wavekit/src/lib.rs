use std::io::Cursor;
use std::path::PathBuf;

pub trait Loadable {
    fn load(self) -> Vec<f32>;
}

impl Loadable for PathBuf {
    fn load(self) -> Vec<f32> {
        let reader = hound::WavReader::open(self).unwrap();
        read_samples(reader)
    }
}

impl<'a> Loadable for &'a [u8] {
    fn load(self) -> Vec<f32> {
        let reader = hound::WavReader::new(Cursor::new(self)).unwrap();
        read_samples(reader)
    }
}

fn read_samples<R: std::io::Read>(mut reader: hound::WavReader<R>) -> Vec<f32> {
    let spec = reader.spec();
    let samples = match spec.sample_format {
        hound::SampleFormat::Float => reader
            .samples::<f32>()
            .map(|s| s.unwrap_or_default())
            .collect::<Vec<_>>(),

        hound::SampleFormat::Int => {
            let bit_depth = spec.bits_per_sample;
            let scaling_factor = 1.0 / (1 << (bit_depth - 1)) as f32;
            reader
                .samples::<i32>()
                .map(|s| s.unwrap_or_default() as f32 * scaling_factor * 0.3)
                .collect::<Vec<_>>()
        },
    };

    samples
}

pub fn load<T: Loadable>(input: T) -> Vec<f32> {
    input.load()
}