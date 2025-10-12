use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ci")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Build all packages defined in bundler.toml, including private ones.
    /// Only applies when no subcommand is given.
    #[arg(long, global = true)]
    pub all: bool,

    /// Any arguments not part of the CLI will be passed to cargo commands.
    /// For example: `ci --release` or `ci bundle -p my-plugin --release`.
    /// Must be the LAST argument.
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub extra_args: Vec<String>,
}

#[derive(Args)]
pub struct BundleArgs {
    /// The primary package to bundle.
    pub name: Option<String>,

    /// Package(s) to bundle via the -p or --package flags.
    #[arg(short = 'p', long = "package")]
    pub packages: Vec<String>,

    /// Any arguments not part of the subcommand will be passed to cargo.
    /// For example: `--release`. Must be the LAST argument.
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub extra_args: Vec<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    Fmt,
    Bundle(BundleArgs),
}
