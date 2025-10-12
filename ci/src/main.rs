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

    /// Build all packages defined in bundler.toml, including private ones.
    /// Only applies when no subcommand is given.
    #[arg(long, global = true)]
    all: bool,

    /// Any arguments not part of the CLI will be passed to cargo commands.
    /// For example: `ci --release` or `ci bundle -p my-plugin --release`.
    /// Must be the LAST argument.
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    extra_args: Vec<String>,
}

#[derive(Args)]
struct BundleArgs {
    /// The primary package to bundle.
    name: Option<String>,

    /// Package(s) to bundle via the -p or --package flags.
    #[arg(short = 'p', long = "package")]
    packages: Vec<String>,

    /// Any arguments not part of the subcommand will be passed to cargo.
    /// For example: `--release`. Must be the LAST argument.
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    extra_args: Vec<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Format the code using `cargo fmt` and `cargo sort`.
    Fmt,
    /// Build and bundle one or more specified packages.
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
            // Collect all packages specified either as the main arg or with -p/--package
            let packages: Vec<_> = args
                .packages
                .into_iter()
                .chain(args.name.into_iter())
                .collect();

            if packages.is_empty() {
                return Err(Error::msg(
                    "No packages specified for bundling. Use `-p <package_name>` or provide a name.",
                ));
            }

            // The 'bundle' subcommand always bundles the specified packages,
            // regardless of the 'private' setting in bundler.toml.
            println!("Explicitly bundling: {:?}", packages);
            build_and_bundle(&packages, &args.extra_args, target_dir)?;
        }
        None => {
            // Default command: build all non-private packages, or all if --all is
            // specified.
            println!("Running default build and bundle...");
            if let Some(config) = load_bundler_config()? {
                let packages_to_bundle: Vec<String> = config
                    .into_iter()
                    .filter(|(_name, pkg_config)| {
                        // If --all is passed, bundle everything.
                        // Otherwise, only bundle packages that are not explicitly private.
                        cli.all || !pkg_config.private.unwrap_or(false)
                    })
                    .map(|(name, _config)| name)
                    .collect();

                if cli.all {
                    println!("--all flag specified, bundling all packages from bundler.toml.");
                } else {
                    println!("Bundling non-private packages from bundler.toml.");
                }

                build_and_bundle(&packages_to_bundle, &cli.extra_args, target_dir)?;
            } else {
                println!("'bundler.toml' not found. Nothing to do for the default command.");
            }
        }
    }

    Ok(())
}

fn build_and_bundle(packages: &[String], extra_args: &[String], target_dir: &Path) -> Result<()> {
    if packages.is_empty() {
        println!("No packages selected to build and bundle.");
        return Ok(());
    }

    println!(
        "Building packages: {:?} with args: {:?}",
        packages, extra_args
    );
    nih_plug_xtask::build(packages, extra_args)?;

    for pkg in packages {
        println!("Bundling package: {} with args: {:?}", pkg, extra_args);
        nih_plug_xtask::bundle(target_dir, pkg, extra_args, false)?;
    }

    println!("✅ Done.");
    Ok(())
}

fn run_fmt() -> Result<()> {
    println!("Running formatters...");
    // Run `cargo fmt`
    let status = Command::new("cargo").arg("fmt").status()?;
    if !status.success() {
        return Err(anyhow::Error::msg("`cargo fmt` failed"));
    }

    // Run `cargo sort -w`
    if let Err(e) = run_optional_command("cargo", &["sort", "-w"], "cargo-sort") {
        println!("Note: {}", e);
    }

    // Run `cargo sort-derives`
    if let Err(e) = run_optional_command("cargo", &["sort-derives"], "cargo-sort-derives") {
        println!("Note: {}", e);
    }

    println!("✅ Formatting complete.");
    Ok(())
}

/// Tries to run a command, and if it's not installed, returns a friendly
/// message.
fn run_optional_command(cmd: &str, args: &[&str], name: &str) -> Result<(), String> {
    match Command::new(cmd).args(args).status() {
        Ok(status) if status.success() => Ok(()),
        Ok(_) => Err(format!("`{} {:?}` failed", cmd, args.join(" "))),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Err(format!(
            "{} is not installed. Please install it to use this feature.",
            name
        )),
        Err(e) => Err(format!("Failed to run {}: {}", name, e)),
    }
}

fn load_bundler_config() -> Result<Option<BundlerConfig>> {
    let bundler_config_path = Path::new("bundler.toml");
    if !bundler_config_path.exists() {
        return Ok(None);
    }

    let result = toml::from_str(&fs::read_to_string(bundler_config_path)?)?;
    Ok(Some(result))
}

/// Configuration for creating plugin bundles, stored in `bundler.toml`.
type BundlerConfig = HashMap<String, PackageConfig>;

#[derive(Clone, Debug, Deserialize)]
struct PackageConfig {
    private: Option<bool>,
}
