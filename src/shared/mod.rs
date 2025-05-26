use qrgen::{Mask, QrCode, QrSegment, Version};

use crate::commands::cli::SharedArgs;

pub mod qrgen;
pub mod wifi;

pub trait Qr {
    /// Struct the data to use it in the QR
    fn to_str(&self) -> String;

    /// This will be used to the interactive mode
    fn get_data<T: Qr>(element: T) -> T;
}

pub fn to_qr(data: String, args: SharedArgs) {
    let segments = QrSegment::make_segments(data.as_str());

    let qr = QrCode::encode_segments_advanced(
        &segments,
        args.ecl,
        Version::MIN,
        if let Some(version) = args.max_version {
            Version::new(version)
        } else {
            Version::MAX
        },
        if let Some(mask) = args.mask {
            Some(Mask::new(mask))
        } else {
            None
        },
        false
    );

    if let Err(e) = qr {
        eprintln!("Error generating QR code: {}", e);
        return;
    }

    let qr = qr.unwrap();

    // Print the QR code to the console
    for y in 0..qr.size() {
        for x in 0..qr.size() {
            if qr.get_module(x, y) {
                print!("██");
            } else {
                print!("  ");
            }
        }
        println!();
    }
}
