use cargo_emit::warning;
use npm_rs::{ NodeEnv, NpmEnv };
use std::{ env::VarError, fs::canonicalize, process::ExitStatus };

pub use cargo_emit;

pub fn build(path: &str, profile: Result<String, VarError>) -> Result<ExitStatus, std::io::Error> {
    let node_env = match profile {
        Ok(profile) =>
            match profile.as_str() {
                "debug" => NodeEnv::Development,
                "release" => NodeEnv::Production,
                _ => NodeEnv::default(),
            }
        Err(e) => {
            panic!("Failed to retrieve PROFILE environment variable: {}", e);
        }
    };

    return NpmEnv::default()
        .with_node_env(&node_env)
        .set_path(canonicalize(path).unwrap())
        .init_env()
        .install(None)
        .run("build")
        .exec();
}
