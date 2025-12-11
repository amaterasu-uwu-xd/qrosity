use crate::core::renderer::{
    QrRenderer, eps::EpsRenderer, pdf::PdfRenderer, png::PngRenderer, svg::SvgRenderer,
};
use crate::{
    core::qrgen::{Mask, QrCode, QrSegment, Version},
    models::{OutputFormat, QrItem},
};

pub mod qrgen;
pub mod renderer;

pub use qrgen::QrCodeEcc;

/// Generates a QR code from the given data and configuration.
/// Returns the appropriate renderer containing the generated QR code.
pub fn generate_qr<T: QrItem>(item: &T) -> Result<Box<dyn QrRenderer>, String> {
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
        config.boost_error_correction,
    )
    .map_err(|e| format!("Error generating QR: {:?}", e))?;

    let renderer: Box<dyn QrRenderer> = match config.format {
        OutputFormat::Svg => Box::new(SvgRenderer::new(&qr, config)?),
        OutputFormat::Eps => Box::new(EpsRenderer::new(&qr, config)?),
        OutputFormat::Pdf => Box::new(PdfRenderer::new(&qr, config)?),
        _ => Box::new(PngRenderer::new(&qr, config)?),
    };

    Ok(renderer)
}
