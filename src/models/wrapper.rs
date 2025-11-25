use std::fmt;
use super::WifiQr;

#[cfg(feature = "cli")]
use clap::Subcommand;

#[derive(Debug)]
#[cfg_attr(feature = "cli", derive(Subcommand))]
pub enum QrData {
    Wifi(WifiQr),
}

impl fmt::Display for QrData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QrData::Wifi(wifi) => write!(f, "{}", wifi),
        }
    }
}