use build_function::{ build, cargo_emit::{ rerun_if_changed, warning } };

fn main() {
    rerun_if_changed!(format!("{}/src/", env!("TOYBOX_C1200_UI")), "build.rs");

    match build(env!("TOYBOX_C1200_UI"), std::env::var("PROFILE")) {
        Ok(_) => {
            warning!("Build succeeded");
        }
        Err(e) => {
            panic!("Build failed, {}", e);
        }
    }
}
