use std::fmt;
use super::{ WifiQr, TextQr };

#[cfg(feature = "cli")]
use clap::Subcommand;

#[derive(Debug)]
#[cfg_attr(feature = "cli", derive(Subcommand))]
pub enum QrData {
    /// Generate a QR code from plain text. Recommended for URLs and other text data.
    Text(TextQr),
    /// Generate a QR code for Wi-Fi network configuration.
    Wifi(WifiQr),
}

impl fmt::Display for QrData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QrData::Wifi(wifi) => write!(f, "{}", wifi),
            QrData::Text(text) => write!(f, "{}", text),
        }
    }
}