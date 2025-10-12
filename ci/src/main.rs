use std::{collections::HashMap, fs, path::Path, process::Command};

use anyhow::{Error, Result};
use clap::{Args, Parser, Subcommand};
use nih_plug_xtask::chdir_workspace_root;
use serde::Deserialize;

#[derive(Parser)]
#[command(name = "ci")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Args)]
struct BundleArgs {
    /// The primary package to bundle
    name: Option<String>,

    /// Package(s) to bundle via -p or --package flags
    #[arg(short = 'p', long = "package")]
    packages: Vec<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Format the code
    Fmt,
    /// Bundle one or more packages
    Bundle(BundleArgs),
}

fn main() -> Result<()> {
    chdir_workspace_root()?;
    let cargo_metadata = cargo_metadata::MetadataCommand::new()
        .manifest_path("./Cargo.toml")
        .exec()?;

    let target_dir = cargo_metadata.target_directory.as_std_path();

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Fmt) => run_fmt()?,
        Some(Commands::Bundle(args)) => {
            let packages: Vec<_> = args
                .packages
                .into_iter()
                .chain(args.name.into_iter())
                .collect();

            if packages.is_empty() {
                return Err(Error::msg("No packages specified"));
            }

            let other_args: Vec<String> = Vec::new();
            nih_plug_xtask::build(&packages, &other_args)?;

            for pkg in packages {
                nih_plug_xtask::bundle(target_dir, &pkg, &other_args, false)?;
            }
        }
        None => {
            if let Some(config) = load_bundler_config()? {
                for package in config.keys() {
                    println!("{package} and private is {:?}", config[package].private);
                }
            }
        }
    }

    Ok(())
}

fn run_fmt() -> Result<()> {
    // Run `cargo fmt --all`
    let status = Command::new("cargo").arg("fmt").arg("--all").status()?;

    if !status.success() {
        return Err(anyhow::Error::msg("`cargo fmt` failed"));
    }

    // Run `cargo sort -w`
    if let Err(e) = run_optional_command("cargo", &["sort", "-w"], "cargo-sort") {
        println!("{}", e);
    }

    // Run `cargo sort-derives`
    if let Err(e) = run_optional_command("cargo", &["sort-derives"], "cargo-sort-derives") {
        println!("{}", e);
    }

    Ok(())
}

/// Tries to run a command, and if it's not installed, returns a friendly
/// message
fn run_optional_command(cmd: &str, args: &[&str], name: &str) -> Result<(), String> {
    match Command::new(cmd).args(args).status() {
        Ok(status) if status.success() => Ok(()),
        Ok(_) => Err(format!("`{} {:?}` failed", cmd, args)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            Err(format!("{} is not installed. Please install it.", name))
        }
        Err(e) => Err(format!("Failed to run {}: {}", name, e)),
    }
}

fn load_bundler_config() -> Result<Option<BundlerConfig>> {
    // We're already in the project root
    let bundler_config_path = Path::new("bundler.toml");
    if !bundler_config_path.exists() {
        return Ok(None);
    }

    let result = toml::from_str(&fs::read_to_string(bundler_config_path)?)?;

    Ok(Some(result))
}

/// Any additional configuration that might be useful for creating plugin
/// bundles, stored as `bundler.toml` alongside the workspace's main
/// `Cargo.toml` file.
type BundlerConfig = HashMap<String, PackageConfig>;

#[derive(Debug, Clone, Deserialize)]
struct PackageConfig {
    name: Option<String>,
    private: Option<bool>,
}
