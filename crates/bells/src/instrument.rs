use std::sync::Arc;

use rkyv::{Archive, Deserialize, Serialize};
use rustc_hash::FxHashMap;
use zstd::{decode_all, encode_all};

// The serialized version on disk is still a plain FxHashMap<u8, Vec<f32>>.
#[derive(Debug, Serialize, Deserialize, Archive, Default)]
struct SerializableInstrument {
    name: String,
    samples: FxHashMap<u8, Vec<f32>>,
}

// The in-memory version we use in the plugin.
#[derive(Debug, Default)]
pub struct Instrument {
    pub name: String,
    pub samples: FxHashMap<u8, Arc<Vec<f32>>>,
}

impl Instrument {
    #[allow(dead_code)]
    pub fn encode(instr: Instrument) -> Vec<u8> {
        let serializable = SerializableInstrument {
            name: instr.name,
            samples: instr
                .samples
                .into_iter()
                .map(|(k, v)| (k, v.as_ref().clone()))
                .collect(),
        };
        let encoded = rkyv::to_bytes::<_, 256>(&serializable).unwrap();
        encode_all(encoded.as_ref(), 1).unwrap()
    }

    /// Decodes a compressed binary vector back into an Instrument struct.
    pub fn decode(bin: Vec<u8>) -> Instrument {
        let decoded = decode_all(bin.as_slice()).unwrap();
        let serializable: SerializableInstrument =
            unsafe { rkyv::from_bytes_unchecked(&decoded[..]).unwrap() };

        // Convert the loaded Vecs into Arcs for efficient sharing.
        Instrument {
            name: serializable.name,
            samples: serializable
                .samples
                .into_iter()
                .map(|(k, v)| (k, Arc::new(v)))
                .collect(),
        }
    }
}
