use clap::{Parser, Subcommand};

#[cfg(feature = "gui")]
use qrosity::modes::gui::run as run_gui_app;

#[cfg(feature = "cli")]
use qrosity::{models::{QrConfig, QrData, TextQr}, modes::cli::run as run_cli_app};

#[derive(Parser)]
#[command(author, version, about = "Qrosity - QR Code Generator")]
struct App {
    #[cfg(feature = "cli")]
    #[command(flatten)]
    config: QrConfig,

    #[cfg(feature = "cli")]
    #[command(flatten)]
    text: TextQr,

    #[command(subcommand)]
    mode: Option<AppMode>,
}

#[derive(Subcommand)]
enum AppMode {
    #[cfg(feature = "cli")]
    #[command(flatten)]
    Qr(QrData),

    #[cfg(feature = "gui")]
    Gui,

    #[cfg(feature = "batch")]
    Batch {
        #[arg(help = "Input file path")]
        input: String,
    }
}

fn main() {

    let app = App::parse();

    match app.mode {
        #[cfg(feature = "gui")]
        Some(AppMode::Gui) => {
            run_gui_app();
        },
        #[cfg(feature = "batch")]
        Some(AppMode::Batch { input }) => {
            // Placeholder for batch mode functionality
            println!("Batch mode with input file: {}", input);
        },
        #[cfg(feature = "cli")]
        Some(AppMode::Qr(data)) => {
            run_cli_app(app.config, data);
        },
        None => {
            #[cfg(feature = "cli")]
            run_cli_app(app.config, QrData::Text(app.text));
        }
    }
}
