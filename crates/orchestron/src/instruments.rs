use std::collections::HashMap;

pub struct Instrument {
    pub notes: HashMap<u8, Vec<f32>>,
}