use super::{EmailQr, QrConfig, QrItem, TextQr, WifiQr};
use std::fmt;

#[cfg(feature = "cli")]
use clap::Subcommand;

/// Enum representing different types of QR code data.
/// Each variant holds the corresponding data structure for that QR code type.
#[derive(Debug)]
#[cfg_attr(feature = "cli", derive(Subcommand))]
#[cfg_attr(feature = "batch", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "batch", serde(rename_all = "kebab-case"))]
pub enum QrData {
    /// Generate a QR code from plain text. Recommended for URLs and other text data.
    #[cfg_attr(feature = "cli", command(skip))]
    Text(TextQr),
    /// Generate a QR code for Wi-Fi network configuration.
    Wifi(WifiQr),
    /// Generate a QR code for sending an email.
    Email(EmailQr),
}

impl QrItem for QrData {
    fn config(&self) -> &QrConfig {
        match self {
            QrData::Text(t) => t.config(),
            QrData::Wifi(w) => w.config(),
            QrData::Email(e) => e.config(),
        }
    }
}

impl fmt::Display for QrData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QrData::Wifi(wifi) => write!(f, "{}", wifi),
            QrData::Text(text) => write!(f, "{}", text),
            QrData::Email(email) => write!(f, "{}", email),
        }
    }
}
