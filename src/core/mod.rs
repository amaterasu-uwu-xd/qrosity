use crate::{core::qrgen::{Mask, QrCode, QrSegment, Version}, models::QrItem};

mod qrgen;
mod renderer;

pub use qrgen::QrCodeEcc;

/// Generates a QR code from the given data and configuration,
/// then renders and saves it in the specified output format.
/// The output format is determined by the file extension
/// in the `config.output` field (e.g., ".png" or ".svg").
pub fn to_qr<T: QrItem>(item: T) {
    let content = item.to_string();
    let config = item.config();

    let segments = QrSegment::make_segments(&content);
    let qr = QrCode::encode_segments_advanced(
        &segments, 
        config.ecl,
        Version::MIN,
        if let Some(version) = config.max_version {
            Version::new(version)
        } else {
            Version::MAX
        },
        if let Some(mask) = config.mask {
            Some(Mask::new(mask))
        } else {
            None
        },
        config.boost_error_correction
    );

    let qr = qr.unwrap();

    // Check output format
    let extension = std::path::Path::new(&config.output)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_lowercase());

    match extension.as_deref() {
        Some("svg") => {
            match renderer::svg::render_svg(&qr, &config, config.ppm as f32) {
                Ok(svg_content) => {
                    if let Err(e) = std::fs::write(&config.output, svg_content) {
                        eprintln!("Error saving SVG: {}", e);
                    } else {
                        println!("QR code saved to {}", &config.output);
                    }
                },
                Err(e) => eprintln!("Error rendering SVG: {}", e),
            }
        },
        Some("eps") => {
            match renderer::eps::render_eps(&qr, config, config.ppm as f32) {
                Ok(eps_content) => {
                    if let Err(e) = std::fs::write(&config.output, eps_content) {
                        eprintln!("Error saving EPS: {}", e);
                    } else {
                        println!("QR code saved to {}", &config.output);
                    }
                },
                Err(e) => eprintln!("Error rendering EPS: {}", e),
            }
        },
        _ => {
            // Call the raster renderer
            match renderer::png::render_qr(&qr, &config, config.ppm as f32) {
                Ok(pixmap) => {
                    if let Err(e) = renderer::png::save_image(&pixmap, &config.output) {
                        eprintln!("{}", e);
                    } else {
                        println!("QR code saved to {}", &config.output);
                    }
                },
                Err(e) => eprintln!("Error rendering QR: {}", e),
            }
        }
    }
}