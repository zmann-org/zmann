use std::sync::Arc;

use rkyv::{Archive, Deserialize, Serialize};
use zstd::{decode_all, encode_all};

#[derive(Debug, Serialize, Deserialize, Archive, Default)]
pub struct Instrument {
    pub name: String,
    pub sample: Arc<Vec<f32>>,
}

#[derive(Debug, Serialize, Deserialize, Archive)]
struct SerializableInstrument {
    name: String,
    pub sample: Vec<f32>,
}

impl Instrument {
    pub fn encode(instr: Instrument) -> Vec<u8> {
        let serializable = SerializableInstrument {
            name: instr.name,
            sample: instr.sample.as_ref().clone(),
        };
        let encoded = rkyv::to_bytes::<_, 256>(&serializable).unwrap();
        encode_all(encoded.as_ref(), 1).unwrap()
    }

    pub fn decode(bin: Vec<u8>) -> Instrument {
        let decoded = decode_all(bin.as_slice()).unwrap();
        let serializable: SerializableInstrument =
            unsafe { rkyv::from_bytes_unchecked(&decoded[..]).unwrap() };

        Instrument {
            name: serializable.name,
            sample: Arc::new(serializable.sample),
        }
    }
}
