use nih_plug::prelude::Enum;
use serde::{ Deserialize, Serialize };
use strum::Display;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, Enum, Display)]
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
