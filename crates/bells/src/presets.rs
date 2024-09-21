use nih_plug::prelude::Enum;
use strum::Display;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Enum, Display)]
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

    pub fn content(&self) -> &[u8] {
        match self {
            Presets::Brass => BRASS,
            Presets::Plastic => PLASTIC,
        }
    }
}
