use clap::{Args, Parser, Subcommand};

#[cfg(feature = "gui")]
use qrosity::modes::gui::run as run_gui_app;

#[cfg(feature = "cli")]
use qrosity::{
    models::{QrData, TextQr},
    modes::cli::run as run_cli_app,
};

#[cfg(feature = "batch")]
use qrosity::modes::batch::run as run_batch_app;

/// Main application structure for qrosity.
#[derive(Parser)]
#[command(author, version, about = "Qrosity - QR Code Generator")]
struct App {
    /// Text input for QR code generation
    #[cfg(feature = "cli")]
    #[command(flatten)]
    text: TextQr,

    /// Command-line only options
    #[cfg(feature = "cli")]
    #[command(flatten)]
    options: CliOptions,

    #[command(subcommand)]
    mode: Option<AppMode>,
}

/// Command-line only options, used by the default mode when no subcommand is provided
/// and the `cli` feature is enabled.
#[cfg(feature = "cli")]
#[derive(Args)]
struct CliOptions {
    #[arg(
        long,
        short,
        help = "Output file path",
        default_value_t = chrono::Local::now().format("qr_%Y-%m-%d_%H:%M:%S").to_string(),
    )]
    output: String,
}

/// A second set of command-line only options for the generate subcommand.
/// If `global = true` is used in `CliOptions`, the output option will be available
/// for all subcommands in the same jerarchy, like `batch` and `gui`, who do not need it.
#[cfg(feature = "cli")]
#[derive(Args)]
struct GenerateOptions {
    #[arg(
        long,
        short,
        help = "Output file path",
        default_value_t = chrono::Local::now().format("qr_%Y-%m-%d_%H:%M:%S").to_string(),
        global = true,
    )]
    output: String,
}

/// Enum representing the different application modes: CLI, GUI, and Batch processing.
#[derive(Subcommand)]
enum AppMode {
    #[cfg(feature = "cli")]
    /// A set of subcommands for generating different types of QR codes.
    Generate {
        #[command(subcommand)]
        data: QrData,
        #[command(flatten)]
        options: GenerateOptions,
    },

    /// Launch the graphical user interface for QR code generation.
    #[cfg(feature = "gui")]
    Gui,

    /// Batch processing mode for generating multiple QR codes from a JSON file.
    #[cfg(feature = "batch")]
    Batch {
        #[arg(help = "Input file path")]
        input: String,

        #[arg(
            long,
            short,
            help = "Maximum number of threads to use",
            default_value_t = 4
        )]
        threads: usize,
    },
}

fn main() {
    let app = App::parse();

    match app.mode {
        #[cfg(feature = "gui")]
        Some(AppMode::Gui) => {
            run_gui_app();
        }
        #[cfg(feature = "batch")]
        Some(AppMode::Batch { input, threads }) => {
            run_batch_app(input, threads);
        }
        #[cfg(feature = "cli")]
        Some(AppMode::Generate { data, options }) => {
            run_cli_app(data, options.output);
        }
        None => {
            #[cfg(feature = "cli")]
            run_cli_app(QrData::Text(app.text), app.options.output);
        }
    }
}
