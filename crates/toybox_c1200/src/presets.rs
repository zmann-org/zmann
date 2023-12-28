use nih_plug::prelude::Enum;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, Enum, Display)]
pub enum Presets {
    Accordion,
    AltoSax,
    Bandoneon,
    Brass1,
    Brass2,
    BrassEnsemble,
    Cello,
    ChurchOrgan,
    Clarinet,
    ElecOrgan1,
    ElecOrgan2,
    ElecOrgan3,
    ElecOrgan4,
    Flute,
    FrenchHorn1,
    FrenchHorn2,
    Harmonica,
    Harp,
    Oboe,
    Piccolo,
    PipeOrgan,
    Recorder,
    ReedOrgan,
    SopranoSax,
    Soundtrack,
    Strings1,
    Strings2,
    Strings3,
    SynPad1,
    SynPad2,
    SynPad3,
    TenorSax,
    Trumpet,
    Tuba,
    Violin,
}

impl Default for Presets {
    fn default() -> Self {
        Self::Cello
    }
}
