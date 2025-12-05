use std::fmt;

#[cfg(feature = "cli")]
use clap::{Args, ValueEnum};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "cli", derive(ValueEnum))]
#[cfg_attr(feature = "batch", derive(serde::Serialize, serde::Deserialize))]
pub enum WifiSecurity {
    WPA,
    WEP,
    NoPass,
}

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
    #[cfg_attr(feature = "cli", arg(long, help = "Whether the WiFi network is hidden"))]
    pub hidden: bool,
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