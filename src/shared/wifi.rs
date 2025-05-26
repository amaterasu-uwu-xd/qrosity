use clap::Args;

use crate::{commands::cli::SharedArgs, shared::Qr};

use super::to_qr;

#[derive(Args)]
pub struct WiFi {
    /// Network name
    #[arg(long)]
    pub ssid: String,

    /// Network password
    #[arg(long)]
    pub password: String,

    /// Network auth type
    #[arg(long, default_value = "WPA", value_parser = ["WPA", "WEP"])]
    pub encryption: String,

    #[clap(flatten)]
    pub shared: SharedArgs,
}

pub fn handle(cmd: WiFi) {
    let qr_str = cmd.to_str();
    to_qr(qr_str, cmd.shared);
}

impl Qr for WiFi {
    fn to_str(&self) -> String {
        format!("WIFI:S:{};T:{};P:{};;", self.ssid, self.encryption, self.password)
    }
    
    fn get_data<T: Qr>(element: T) -> T {
        todo!()
    }
}