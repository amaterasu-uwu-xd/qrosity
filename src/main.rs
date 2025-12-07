use clap::{Parser, Subcommand};

#[cfg(feature = "gui")]
use qrosity::modes::gui::run as run_gui_app;

#[cfg(feature = "cli")]
use qrosity::{models::{QrData, TextQr}, modes::cli::run as run_cli_app};

#[cfg(feature = "batch")]
use qrosity::modes::batch::run as run_batch_app;

/// Main application structure for qrosity.
#[derive(Parser)]
#[command(author, version, about = "Qrosity - QR Code Generator")]
struct App {
    #[cfg(feature = "cli")]
    #[command(flatten)]
    text: TextQr,

    #[command(subcommand)]
    mode: Option<AppMode>,
}

/// Enum representing the different application modes: CLI, GUI, and Batch processing.
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

        #[arg(long, short, help = "Maximum number of threads to use", default_value_t = 4)]
        threads: usize,
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
        Some(AppMode::Batch { input, threads }) => {
            run_batch_app(input, threads);
        },
        #[cfg(feature = "cli")]
        Some(AppMode::Qr(data)) => {
            run_cli_app(data);
        },
        None => {
            #[cfg(feature = "cli")]
            run_cli_app(QrData::Text(app.text));
        }
    }
}
