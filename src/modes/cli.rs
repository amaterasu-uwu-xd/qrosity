use std::io::{self, Read, IsTerminal};
use crate::{core::to_qr, models::QrData};

/// Runs the CLI mode.
/// Processes the provided QrData, reading from stdin if necessary, and generates the QR code.
pub fn run(mut data: QrData) {
    if let QrData::Text(ref mut text_qr) = data {
        if text_qr.text.is_none() {
            if !io::stdin().is_terminal() {
                let mut buffer = String::new();
                match io::stdin().read_to_string(&mut buffer) {
                    Ok(_) => {
                        text_qr.text = Some(buffer.trim().to_string());
                    },
                    Err(e) => {
                        eprintln!("Error reading from stdin: {}", e);
                        std::process::exit(1);
                    }
                }
            } else {
                eprintln!("Error: Text argument is required or input must be piped.");
                std::process::exit(1);
            }
        }
    }

    let content = data.to_string();
    if content.is_empty() {
         eprintln!("Error: No content to encode.");
         std::process::exit(1);
    }

    to_qr(data);
}