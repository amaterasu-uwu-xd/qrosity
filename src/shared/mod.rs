use qrgen::{Mask, QrCode, QrSegment, Version};

use crate::commands::cli::SharedArgs;

pub mod qrgen;
pub mod wifi;
pub mod text;
pub mod mail;
pub mod web;

pub trait Qr {
    /// Struct the data to use it in the QR
    fn to_str(&self) -> String;
    /// This will be used to the interactive mode
    fn get_data<T: Qr>() -> T;
}

pub fn to_qr(data: String, args: SharedArgs) {
    let segments = QrSegment::make_segments(data.as_str(), );

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

    let svg = to_svg_string(&qr, args.border);

    // Save the SVG to the specified output file
    if let Some(output) = args.output.to_str() {
        std::fs::write(output, svg).expect("Unable to write SVG file");
        println!("QR code saved to {}", output);
    } else {
        eprintln!("Invalid output path");
    }
}

// Returns a string of SVG code for an image depicting
// the given QR Code, with the given number of border modules.
// The string always uses Unix newlines (\n), regardless of the platform.
fn to_svg_string(qr: &QrCode, border: i32) -> String {
	assert!(border >= 0, "Border must be non-negative");
	let mut result = String::new();
	result += "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n";
	result += "<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">\n";
	let dimension = qr.size().checked_add(border.checked_mul(2).unwrap()).unwrap();
	result += &format!(
		"<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" viewBox=\"0 0 {0} {0}\" stroke=\"none\">\n", dimension);
	result += "\t<rect width=\"100%\" height=\"100%\" fill=\"#FFFFFF\"/>\n";
	result += "\t<path d=\"";
	for y in 0 .. qr.size() {
		for x in 0 .. qr.size() {
			if qr.get_module(x, y) {
				if x != 0 || y != 0 {
					result += " ";
				}
				result += &format!("M{},{}h1v1h-1z", x + border, y + border);
			}
		}
	}
	result += "\" fill=\"#000000\"/>\n";
	result += "</svg>\n";
	result
}