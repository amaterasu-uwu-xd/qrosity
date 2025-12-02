use std::fmt;
use super::{ WifiQr, TextQr, EmailQr };

#[cfg(feature = "cli")]
use clap::Subcommand;

#[derive(Debug)]
#[cfg_attr(feature = "cli", derive(Subcommand))]
pub enum QrData {
    /// Generate a QR code from plain text. Recommended for URLs and other text data.
    #[cfg_attr(feature = "cli", command(skip))]
    Text(TextQr),
    /// Generate a QR code for Wi-Fi network configuration.
    Wifi(WifiQr),
    /// Generate a QR code for sending an email.
    Email(EmailQr),
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