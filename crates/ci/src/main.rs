use anyhow::{Error, Result};
use clap::Parser;
use nih_plug_xtask::chdir_workspace_root;

mod cli;
mod commands;
mod config;
mod util;

use cli::{Cli, Commands};

fn main() -> Result<()> {
    chdir_workspace_root()?;
    let cargo_metadata = cargo_metadata::MetadataCommand::new()
        .manifest_path("./Cargo.toml")
        .exec()?;
    let target_dir = cargo_metadata.target_directory.as_std_path();

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Fmt) => commands::run_fmt(),
        Some(Commands::Bundle(args)) => {
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

            commands::build_and_bundle(&packages, &args.extra_args, target_dir)
        }
        None => commands::run_default(cli.all, &cli.extra_args, target_dir),
    }
}
