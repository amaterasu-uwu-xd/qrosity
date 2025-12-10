use super::{QrConfig, QrItem};
use std::fmt;

#[cfg(feature = "cli")]
use clap::{Args, ValueEnum};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "cli", derive(ValueEnum))]
#[cfg_attr(feature = "batch", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "batch", serde(rename_all = "lowercase"))]
pub enum WifiSecurity {
    WPA,
    WEP,
    NoPass,
}

/// Represents the data needed to generate a QR code for a WiFi network.
/// This includes the SSID, security type, password, and whether the network is hidden.
#[derive(Debug)]
#[cfg_attr(feature = "cli", derive(Args))]
#[cfg_attr(feature = "batch", derive(serde::Serialize, serde::Deserialize))]
pub struct WifiQr {
    #[cfg_attr(feature = "cli", arg(long, help = "SSID of the WiFi network"))]
    pub ssid: String,
    #[cfg_attr(feature = "cli", arg(long, help = "Security type of the WiFi network"))]
    pub security: WifiSecurity,
    #[cfg_attr(feature = "cli", arg(long, help = "Password for the WiFi network"))]
    pub password: Option<String>,
    #[cfg_attr(feature = "cli", arg(long, help = "Is the network hidden?"))]
    pub hidden: bool,

    #[cfg_attr(feature = "cli", command(flatten))]
    #[cfg_attr(feature = "batch", serde(flatten))]
    pub config: QrConfig,
}

impl fmt::Display for WifiQr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let security_str = match self.security {
            WifiSecurity::WPA => "WPA",
            WifiSecurity::WEP => "WEP",
            WifiSecurity::NoPass => "nopass",
        };

        let password_str = match &self.password {
            Some(pwd) => pwd.as_str(),
            None => "",
        };

        let hidden_str = if self.hidden { "true" } else { "false" };

        write!(
            f,
            "WIFI:T:{};S:{};P:{};H:{};;",
            security_str, self.ssid, password_str, hidden_str
        )
    }
}

impl QrItem for WifiQr {
    fn config(&self) -> &QrConfig {
        &self.config
    }
}
