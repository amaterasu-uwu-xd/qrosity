use std::path::PathBuf;

use clap::{Args, Subcommand};
use crate::shared::mail::Mail;
use crate::shared::qrgen::QrCodeEcc;
use crate::shared::Qr;
use crate::shared::text::Text;
use crate::shared::wifi::WiFi;

#[derive(Subcommand)]
pub enum CliCommand {
    /// Create a new QR to share a WiFi network
    Wifi(WiFi),
    /// Create a new QR to share text
    Text(Text),
    /// Create a new QR to share an email
    Mail(Mail),
}

pub fn handle(cmd: CliCommand) {
    match cmd {
        CliCommand::Wifi(m) => crate::shared::to_qr(m.to_str(), m.shared),
        CliCommand::Text(m) => crate::shared::to_qr(m.to_str(), m.shared),
        CliCommand::Mail(m) => crate::shared::to_qr(m.to_str(), m.shared)
    };
}

#[derive(Args)]
pub struct SharedArgs {
    /// Border size in modules
    #[arg(long, default_value = "4", value_parser = clap::value_parser!(i32).range(0..=100))]
    pub border: i32,

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
    #[arg(long)]
    pub output: PathBuf,
}
