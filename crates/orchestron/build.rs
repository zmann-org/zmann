use build_function::{ build, cargo_emit::{ rerun_if_changed, warning } };

fn main() {
    rerun_if_changed!("build.rs", format!("{}/src/", env!("ORCHESTRON_UI")));

    match build(env!("ORCHESTRON_UI"), std::env::var("PROFILE")) {
        Ok(_) => {
            warning!("Build succeeded");
        }
        Err(e) => {
            panic!("Build failed, {}", e);
        }
    }
}