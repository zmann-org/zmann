use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Deserialize, Serialize, Debug)]
pub enum PlayingStyle {
    WhilePressed,
    WhileTrigger,
    FadeOut,
}

#[derive(Archive, Deserialize, Serialize, Debug)]
pub struct Instrument {
    pub f0: Vec<f32>,
    pub f1: Vec<f32>,
    pub c2: Vec<f32>,
    pub c3: Vec<f32>,
}

impl Instrument {
    pub fn default() -> Self {
        Instrument {
            f0: vec![],
            f1: vec![],
            c2: vec![],
            c3: vec![],
        }
    }
}

pub fn encode(instr: Instrument) -> Vec<u8> {
    let encoded: &[u8] = &rkyv::to_bytes::<_, 256>(&instr).unwrap();
    return zstd::encode_all(encoded, 1).unwrap();
}

pub fn decode(bin: Vec<u8>) -> Instrument {
    let decoded: Vec<u8> = zstd::decode_all(bin.as_slice()).unwrap();
    unsafe {
        return rkyv::from_bytes_unchecked(&decoded[..]).unwrap();
    }
}
