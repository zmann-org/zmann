use nih_plug::prelude::Enum;
use strum::Display;

#[derive(Clone, Debug, Display, Enum, Eq, Hash, PartialEq)]
pub enum Presets {
    Cello,
    Choir,
    Flute,
    Hammond,
    Horns,
    Organ,
    Saxophone,
    Violins,
}

pub const CELLO: &[u8] = include_bytes!(concat!(env!("SAMPLES"), "orchestron/cello"));
pub const CHOIR: &[u8] = include_bytes!(concat!(env!("SAMPLES"), "orchestron/choir"));
pub const FLUTE: &[u8] = include_bytes!(concat!(env!("SAMPLES"), "orchestron/flute"));
pub const HAMMOND: &[u8] = include_bytes!(concat!(env!("SAMPLES"), "orchestron/hammond"));
pub const HORNS: &[u8] = include_bytes!(concat!(env!("SAMPLES"), "orchestron/horns"));
pub const ORGAN: &[u8] = include_bytes!(concat!(env!("SAMPLES"), "orchestron/organ"));
pub const SAXOPHONE: &[u8] = include_bytes!(concat!(env!("SAMPLES"), "orchestron/saxophone"));
pub const VIOLINS: &[u8] = include_bytes!(concat!(env!("SAMPLES"), "orchestron/violins"));

impl Presets {
    pub fn content(&self) -> &[u8] {
        match self {
            Presets::Cello => CELLO,
            Presets::Choir => CHOIR,
            Presets::Flute => FLUTE,
            Presets::Hammond => HAMMOND,
            Presets::Horns => HORNS,
            Presets::Organ => ORGAN,
            Presets::Saxophone => SAXOPHONE,
            Presets::Violins => VIOLINS,
        }
    }
}

impl Default for Presets {
    fn default() -> Self {
        Presets::Cello
    }
}
