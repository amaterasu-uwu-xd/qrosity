use crate::{core::qrgen::{Mask, QrCode, QrSegment, Version}, models::QrConfig};

pub mod qrgen;
mod renderer;

pub fn to_qr(data: String, config: QrConfig) {
    let segments = QrSegment::make_segments(&data);
    let qr = QrCode::encode_segments_advanced(
        &segments, 
        config.ecl,
        Version::MIN,
        if let Some(version) = config.max_version {
            Version::new(version)
        } else {
            Version::MAX
        },
        if let Some(mask) = config.mask_pattern {
            Some(Mask::new(mask))
        } else {
            None
        },
        config.boost_error_correction
    );

    let qr = qr.unwrap();

    // Check output format
    if config.output.to_lowercase().ends_with(".svg") {
        #[cfg(feature = "svg")]
        {
            match renderer::svg::render_svg(&qr, &config, config.ppm as f32) {
                Ok(document) => {
                    if let Err(e) = svg::save(&config.output, &document) {
                        eprintln!("Error saving SVG: {}", e);
                    } else {
                        println!("QR code saved to {}", &config.output);
                    }
                },
                Err(e) => eprintln!("Error rendering SVG: {}", e),
            }
        }
        #[cfg(not(feature = "svg"))]
        {
            eprintln!("Error: SVG support is not enabled. Compile with --features svg");
        }
    } else {
        // Call the raster renderer
        match renderer::png::render_qr(&qr, &config, config.ppm as f32) {
            Ok(pixmap) => {
                if let Err(e) = pixmap.save_png(&config.output) {
                    eprintln!("Error saving QR code: {}", e);
                } else {
                    println!("QR code saved to {}", &config.output);
                }
            },
            Err(e) => eprintln!("Error rendering QR: {}", e),
        }
    }
}