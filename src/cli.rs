use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: crate::commands::TopLevelCommand,
}
