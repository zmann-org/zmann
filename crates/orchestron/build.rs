use build_function::build;
use cargo_emit::{ rerun_if_changed, warning };

fn main() {
    rerun_if_changed!(format!("{}/src/", env!("ORCHESTRON_UI")), "build.rs");

    match build(env!("ORCHESTRON_UI"), std::env::var("PROFILE")) {
        Ok(_) => {
            warning!("Build succeeded");
        }
        Err(e) => {
            panic!("Build failed, {}", e);
        }
    }
}
