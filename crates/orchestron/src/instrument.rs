use std::collections::HashMap;
use std::sync::Arc;

use rkyv::{Archive, Deserialize, Serialize};
use zstd::{decode_all, encode_all};

#[derive(Debug, Default)]
pub struct Instrument {
    pub samples: HashMap<u8, Arc<Vec<f32>>>,
}

impl Instrument {
    pub fn clear(&mut self) {
        self.samples.clear();
    }
}

#[derive(Debug, Serialize, Deserialize, Archive)]
struct SerializableInstrument {
    name: String,
    pub sample: Vec<f32>,
}

pub fn encode(name: String, sample: Vec<f32>) -> Vec<u8> {
    let serializable = SerializableInstrument { name, sample };
    let encoded = rkyv::to_bytes::<_, 256>(&serializable).unwrap();
    encode_all(encoded.as_ref(), 1).unwrap()
}

pub fn decode(bin: Vec<u8>) -> Vec<f32> {
    let decoded = decode_all(bin.as_slice()).unwrap();
    let serializable: SerializableInstrument =
        unsafe { rkyv::from_bytes_unchecked(&decoded[..]).unwrap() };
    serializable.sample
}
