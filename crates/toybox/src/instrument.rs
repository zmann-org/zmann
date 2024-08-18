use rkyv::{Archive, Deserialize, Serialize};
use rustc_hash::FxHashMap;
use zstd::{decode_all, encode_all};

#[derive(Archive, Deserialize, Serialize, Debug)]
pub struct Instrument {
    pub notes: FxHashMap<u8, Vec<f32>>,
    pub name: String,
}

impl Instrument {
    pub fn default() -> Self {
        Instrument {
            notes: FxHashMap::default(),
            name: String::new(),
        }
    }
}

pub(crate) fn encode(instr: Instrument) -> Vec<u8> {
    let encoded: &[u8] = &rkyv::to_bytes::<_, 256>(&instr).unwrap();
    encode_all(encoded, 1).unwrap()
}

pub(crate) fn decode(bin: Vec<u8>) -> Instrument {
    let decoded: Vec<u8> = decode_all(bin.as_slice()).unwrap();
    unsafe { rkyv::from_bytes_unchecked(&decoded[..]).unwrap() }
}
