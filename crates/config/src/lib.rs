use std::collections::HashMap;

use serde::Deserialize;

pub type Bundler = HashMap<String, Package>;

#[derive(Clone, Debug, Deserialize)]
pub struct Package {
    pub name: String,
    pub private: Option<bool>,
    pub id: Option<String>,
}

#[cfg(feature = "macro")]
pub use config_macro::{clap_id, vst3_id};
