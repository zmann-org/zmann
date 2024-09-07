use nih_plug::prelude::Enum;
use strum::Display;

/// An enumeration of available presets.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Enum, Display)]
pub enum Presets {
    Brass,
    Plastic
}

pub const BRASS: &[u8] = include_bytes!(concat!(env!("SAMPLES"), "bells/brass"));
pub const PLASTIC: &[u8] = include_bytes!(concat!(env!("SAMPLES"), "bells/plastic"));

impl Presets {
    /// Returns the default preset, which is `Organ`.
    ///
    /// # Returns
    ///
    /// The default preset.
    pub fn default() -> Self {
        Presets::Brass
    }

    /// Returns the byte content associated with the preset.
    ///
    /// # Returns
    ///
    /// A slice of bytes representing the preset content.
    pub fn content(&self) -> &[u8] {
        match self {
            Presets::Brass => BRASS,
            Presets::Plastic => PLASTIC,
        }
    }
}
