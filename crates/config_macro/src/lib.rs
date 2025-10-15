use std::{collections::HashMap, sync::OnceLock};

use proc_macro::TokenStream;
use quote::quote;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
struct Package {
    id: Option<String>,
}

type Bundler = HashMap<String, Package>;

static BUNDLER_CONFIG: OnceLock<Bundler> = OnceLock::new();

fn bundler_config() -> &'static Bundler {
    BUNDLER_CONFIG.get_or_init(|| {
        let metadata = cargo_metadata::MetadataCommand::new()
            .exec()
            .expect("Failed to run `cargo metadata`");

        let path = metadata.workspace_root.join("bundler.toml");
        let content = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("Cannot read {}: {}", path.to_string(), e));

        toml::from_str(&content).unwrap_or_else(|e| panic!("Invalid bundler.toml: {}", e))
    })
}

fn current_crate_id() -> String {
    let name = std::env::var("CARGO_PKG_NAME").expect("CARGO_PKG_NAME not set");
    bundler_config()
        .get(&name)
        .unwrap_or_else(|| panic!("Missing entry for '{}' in bundler.toml", name))
        .id
        .clone()
        .unwrap_or_else(|| panic!("Missing 'id' for '{}' in bundler.toml", name))
}

#[proc_macro]
pub fn clap_id(_input: TokenStream) -> TokenStream {
    let id = current_crate_id();
    quote! { #id }.into()
}

#[proc_macro]
pub fn vst3_id(_input: TokenStream) -> TokenStream {
    let id = current_crate_id();
    let mut bytes = [b'.'; 16];
    let id_bytes = id.as_bytes();
    bytes[..id_bytes.len().min(16)].copy_from_slice(&id_bytes[..id_bytes.len().min(16)]);
    quote! { [ #(#bytes),* ] }.into()
}
