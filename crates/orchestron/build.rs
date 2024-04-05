use std::{fmt::format, fs::{self, canonicalize}};

use build_function::{ build, cargo_emit::{ rerun_if_changed, warning } };

fn main() {
    rerun_if_changed!("build.rs"); // format!("{}/src/", env!("ORCHESTRON_UI"))

    match build(env!("ORCHESTRON_UI"), std::env::var("PROFILE")) {
        Ok(_) => {
            warning!("Build succeeded");
        }
        Err(e) => {
            panic!("Build failed, {}", e);
        }
    }

    warning!("path: {:?}", canonicalize(env!("ORCHESTRON_UI")).unwrap());
    let paths = fs::read_dir(format!("{:?}/dist", canonicalize(env!("ORCHESTRON_UI")))).unwrap();

    for test in paths {
        warning!("dist: {}", test.unwrap().path().display()); 
    }
}
