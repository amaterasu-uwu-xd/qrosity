use std::path::PathBuf;

use clap::{Args, Subcommand};
use crate::shared::qrgen::QrCodeEcc;
pub use crate::shared::wifi::WiFi;
pub use crate::shared::wifi;

#[derive(Subcommand)]
pub enum CliCommand {
    /// Create a new QR to share a WiFi network
    Wifi(WiFi),
}

pub fn handle(cmd: CliCommand) {
    match cmd {
        CliCommand::Wifi(m) => wifi::handle(m),
        //CliCommand::Mode2(m) => cli_mode2::handle(m),
    };
}

#[derive(Args)]
pub struct SharedArgs {
    /// Correction level
    #[arg(long, default_value = "medium")]
    pub ecl: QrCodeEcc,

    /// Set the maximum possible version
    #[arg(long, value_parser = clap::value_parser!(u8).range(1..=40))]
    pub max_version: Option<u8>,

    /// Mask to use, if not specified, it will be automatically determined
    #[arg(long, value_parser = clap::value_parser!(u8).range(0..=7))]
    pub mask: Option<u8>,

    /// Where to save the QR
    #[arg(long, short = 'o')]
    pub output: PathBuf,
}
