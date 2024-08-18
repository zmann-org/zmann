use std::io::Cursor;
use std::path::PathBuf;

use hound::{SampleFormat, WavReader};

/// A trait for types that can be loaded into a vector of f32 samples.
pub trait Loadable {
    /// Loads the data and returns a vector of f32 samples.
    fn load(self) -> Vec<f32>;
}

impl Loadable for PathBuf {
    /// Loads a WAV file from a PathBuf and returns a vector of f32 samples.
    ///
    /// # Panics
    ///
    /// This function will panic if the WAV file cannot be opened or read.
    fn load(self) -> Vec<f32> {
        let reader = WavReader::open(self).unwrap();
        read_samples(reader)
    }
}

impl<'a> Loadable for &'a [u8] {
    /// Loads a WAV file from a byte slice and returns a vector of f32 samples.
    ///
    /// # Panics
    ///
    /// This function will panic if the WAV file cannot be read.
    fn load(self) -> Vec<f32> {
        let reader = WavReader::new(Cursor::new(self)).unwrap();
        read_samples(reader)
    }
}

/// Reads samples from a WAV reader and returns them as a vector of f32.
///
/// # Arguments
///
/// * `reader` - A WAV reader from which to read the samples.
///
/// # Returns
///
/// A vector of f32 samples.
///
/// # Panics
///
/// This function will panic if the samples cannot be read.
fn read_samples<R: std::io::Read>(mut reader: WavReader<R>) -> Vec<f32> {
    let spec = reader.spec();
    let samples = match spec.sample_format {
        SampleFormat::Float => reader
            .samples::<f32>()
            .map(|s| s.unwrap_or_default())
            .collect::<Vec<_>>(),

        SampleFormat::Int => {
            let bit_depth = spec.bits_per_sample;
            let scaling_factor = 1.0 / (1 << (bit_depth - 1)) as f32;
            reader
                .samples::<i32>()
                .map(|s| s.unwrap_or_default() as f32 * scaling_factor * 0.3)
                .collect::<Vec<_>>()
        }
    };

    samples
}

/// Loads a WAV file from a given input and returns a vector of f32 samples.
///
/// # Arguments
///
/// * `input` - An input that implements the `Loadable` trait.
///
/// # Returns
///
/// A vector of f32 samples.
///
/// # Panics
///
/// This function will panic if the input cannot be loaded.
pub fn load<T: Loadable>(input: T) -> Vec<f32> {
    input.load()
}
