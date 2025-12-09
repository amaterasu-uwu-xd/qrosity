use super::{QrConfig, QrItem};

#[cfg(feature = "cli")]
use clap::Args;

/// Represents the data needed to generate a QR code for plain text (e.g., URLs).
#[derive(Debug)]
#[cfg_attr(feature = "cli", derive(Args))]
#[cfg_attr(feature = "batch", derive(serde::Serialize, serde::Deserialize))]
pub struct TextQr {
    #[cfg_attr(feature = "cli", arg(help = "Text to encode in the QR code"))]
    pub text: Option<String>,

    /// Output file path.
    #[cfg_attr(
        feature = "cli",
        arg(
            long,
            short,
            help = "Output file path",
            default_value_t = chrono::Local::now().format("qr_%Y-%m-%d_%H:%M:%S").to_string()
        )
    )]
    pub output: String,

    #[cfg_attr(feature = "cli", command(flatten))]
    #[cfg_attr(feature = "batch", serde(flatten))]
    pub config: QrConfig,
}

impl std::fmt::Display for TextQr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(text) = &self.text {
            write!(f, "{}", text)
        } else {
            Ok(())
        }
    }
}

impl QrItem for TextQr {
    fn config(&self) -> &QrConfig {
        &self.config
    }

    fn output(&self) -> &str {
        &self.output
    }
}
