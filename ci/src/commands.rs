use std::path::Path;
use std::process::Command;

use anyhow::Result;

use crate::config::load_bundler_config;
use crate::util::run_optional_command;

/// The default command: build all non-private packages, or all if --all is
/// specified.
pub fn run_default(all_flag: bool, extra_args: &[String], target_dir: &Path) -> Result<()> {
    if let Some(config) = load_bundler_config()? {
        let packages_to_bundle: Vec<String> = config
            .into_iter()
            .filter(|(_name, pkg_config)| {
                // If --all is passed, bundle everything.
                // Otherwise, only bundle packages that are not explicitly private.
                all_flag || !pkg_config.private.unwrap_or(false)
            })
            .map(|(name, _config)| name)
            .collect();

        build_and_bundle(&packages_to_bundle, extra_args, target_dir)?;
    }

    Ok(())
}

/// Build and then bundle the specified packages.
pub fn build_and_bundle(
    packages: &[String],
    extra_args: &[String],
    target_dir: &Path,
) -> Result<()> {
    if packages.is_empty() {
        return Ok(());
    }

    nih_plug_xtask::build(packages, extra_args)?;

    for pkg in packages {
        nih_plug_xtask::bundle(target_dir, pkg, extra_args, false)?;
    }

    Ok(())
}

/// Run all formatters (`cargo fmt`, `cargo sort`, etc.).
pub fn run_fmt() -> Result<()> {
    // Run `cargo fmt`
    let status = Command::new("cargo").arg("fmt").status()?;
    if !status.success() {
        return Err(anyhow::Error::msg("`cargo fmt` failed"));
    }

    // Run `cargo sort -w`, printing a note to stderr if not installed
    if let Err(e) = run_optional_command("cargo", &["sort", "-w"], "cargo-sort") {
        eprintln!("Note: {}", e);
    }

    // Run `cargo sort-derives`, printing a note to stderr if not installed
    if let Err(e) = run_optional_command("cargo", &["sort-derives"], "cargo-sort-derives") {
        eprintln!("Note: {}", e);
    }

    Ok(())
}
