pub mod gui;
pub mod cli;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum TopLevelCommand {
    /// Start the GUI application
    Gui,
    /// Create a new QR code by sending arguments
    #[command(subcommand)]
    Generate(cli::CliCommand),
    // Add batch processing
}

pub fn handle_commands(cli: crate::cli::Cli) {
    match cli.command {
        TopLevelCommand::Gui => gui::handle(),
        TopLevelCommand::Generate(cmd) => cli::handle(cmd),
    }
}
