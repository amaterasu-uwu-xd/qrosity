use clap::{Parser, Subcommand};

#[cfg(feature = "gui")]
use qrosity::modes::gui::run as run_gui_app;

#[cfg(feature = "cli")]
use qrosity::{models::QrConfig, modes::cli::run as run_cli_app};

#[derive(Parser)]
#[command(author, version, about = "Qrosity - QR Code Generator")]
struct App {
    #[command(subcommand)]
    mode: AppMode,
}

#[derive(Subcommand)]
enum AppMode {
    #[cfg(feature = "cli")]
    #[command(name = "cli")]
    Cli(CliArgs),
    
    #[cfg(feature = "gui")]
    Gui,

    #[cfg(feature = "batch")]
    Batch {
        #[arg(help = "Input file path")]
        input: String,
    }
}

#[cfg(feature = "cli")]
#[derive(clap::Args)]
struct CliArgs {
    #[command(flatten)]
    config: QrConfig,

    #[command(subcommand)]
    data: qrosity::models::QrData,
}

fn main() {

    let app = App::parse();

    match app.mode {
        #[cfg(feature = "cli")]
        AppMode::Cli(args) => {
            run_cli_app(
                args.config,
                args.data
            );
        },
        #[cfg(feature = "gui")]
        AppMode::Gui => {
            run_gui_app();
        },
        #[cfg(feature = "batch")]
        AppMode::Batch { input } => {
            // Placeholder for batch mode functionality
            println!("Batch mode with input file: {}", input);
        },
    }
}
