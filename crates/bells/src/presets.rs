use nih_plug::prelude::Enum;
use strum::Display;

#[derive(Clone, Debug, Display, Enum, Eq, Hash, PartialEq)]
pub enum Presets {
    Brass,
    Plastic,
}

pub const BRASS: &[u8] = include_bytes!(concat!(env!("SAMPLES"), "bells/brass"));
pub const PLASTIC: &[u8] = include_bytes!(concat!(env!("SAMPLES"), "bells/plastic"));

impl Presets {
    pub fn default() -> Self {
        Presets::Brass
    }

    /// Returns the compressed instrument data for the selected preset.
    /// with the corresponding `.bin` files.
    pub fn content(&self) -> &[u8] {
        match self {
            Presets::Brass => BRASS,
            Presets::Plastic => PLASTIC,
        }
    }
}
