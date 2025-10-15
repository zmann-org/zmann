use std::collections::HashMap;

use serde::Deserialize;

pub type Bundler = HashMap<String, Package>;

#[derive(Clone, Debug, Deserialize)]
pub struct Package {
    pub name: String,
    pub private: Option<bool>,
    pub id: Option<String>,
}
