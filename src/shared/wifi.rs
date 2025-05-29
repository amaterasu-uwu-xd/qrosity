use clap::Args;
use crate::{commands::cli::SharedArgs, shared::Qr};

#[derive(Args)]
pub struct WiFi {
    /// Network name
    #[arg(long)]
    pub ssid: String,

    /// Network auth type
    #[arg(long, value_parser = ["WEP", "WPA", "WPA3", "NONE"])]
    pub encryption: String,

    /// Network password
    #[arg(long, required_if_eq("encryption", "WEP"), required_if_eq("encryption", "WPA"), required_if_eq("encryption", "WPA3"))]
    pub password: Option<String>,

    /// Hidden network
    #[arg(long, default_value = "false")]
    pub hidden: bool,

    #[clap(flatten)]
    pub shared: SharedArgs,
}

impl Qr for WiFi {
    fn to_str(&self) -> String {
        if self.encryption == "NONE" {
            return format!("WIFI:S:{};T:{};H:{};", self.ssid, self.encryption, self.hidden);
        }
        format!("WIFI:S:{};T:{};P:{};H:{};", self.ssid, self.encryption, self.password.as_ref().unwrap(), self.hidden)
    }
    
    fn get_data<T: Qr>() -> T {
        todo!()
    }
}