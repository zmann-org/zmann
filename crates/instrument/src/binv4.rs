use rkyv::{Archive, Deserialize, Serialize};
use rustc_hash::FxHashMap;

#[derive(Archive, Deserialize, Serialize, Debug)]
pub enum PlayingStyle {
    WhilePressed,
    WhileTrigger,
    FadeOut,
}

#[derive(Archive, Deserialize, Serialize, Debug)]
pub struct Instrument {
    pub notes: FxHashMap<u8, Vec<f32>>,
    pub name: String,
    pub style: PlayingStyle,
}

impl Instrument {
    pub fn empty() -> Self {
        Instrument {
            notes: FxHashMap::default(),
            name: String::new(),
            style: PlayingStyle::WhilePressed,
        }
    }
}

pub fn encode(instr: Instrument) -> Vec<u8> {
    let encoded: &[u8] = &rkyv::to_bytes::<_, 256>(&instr).unwrap();
    return zstd::encode_all(encoded, 3).unwrap();
}

pub fn decode(bin: Vec<u8>) -> Instrument {
    let decoded: Vec<u8> = zstd::decode_all(bin.as_slice()).unwrap();
    unsafe {
        return rkyv::from_bytes_unchecked(&decoded[..]).unwrap();
    }
}
