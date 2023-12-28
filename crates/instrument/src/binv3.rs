use lazy_static::lazy_static;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

lazy_static! {
    pub static ref COMPRESSION_LEVEL: i32 = 3;
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum PlayingStyle {
    WhilePressed,
    WhileTrigger,
    FadeOut,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Instrument {
    pub notes: HashMap<u8, Vec<f32>>,
    pub name: String,
    pub style: PlayingStyle,
}

impl Instrument {
    pub fn empty() -> Self {
        Instrument {
            notes: HashMap::new(),
            name: String::new(),
            style: PlayingStyle::WhilePressed,
        }
    }
}

pub fn encode(instr: Instrument) -> Vec<u8> {
    let encoded: &[u8] = &bincode::serialize(&instr).unwrap();
    return zstd::encode_all(encoded, *COMPRESSION_LEVEL).unwrap();
}

pub fn decode(bin: Vec<u8>) -> Instrument {
    let decoded: Vec<u8> = zstd::decode_all(bin.as_slice()).unwrap();
    return bincode::deserialize(&decoded[..]).unwrap();
}