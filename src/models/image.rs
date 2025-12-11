use image::DynamicImage;

/// Represents an image that can be embedded in a QR code.
/// This can be either a raster image (PNG, JPEG, etc.) or an SVG vector image.
#[derive(Debug, Clone)]
pub enum QrImage {
    /// A raster image loaded into memory.
    Raster(DynamicImage),
    /// An SVG image represented as an XML string.
    Svg(String),
}

impl QrImage {
    /// Creates a `QrImage` from a file path.
    /// Detects if the file is an SVG or a raster image.
    pub fn load_from_path(path: &str) -> Result<Self, String> {
        let path_buf = std::path::Path::new(path);
        if !path_buf.exists() {
            return Err(format!("File not found: {}", path));
        }

        // Simple check for SVG extension or content
        if crate::core::renderer::utils::is_svg_file(path) {
            let svg_content = std::fs::read_to_string(path)
                .map_err(|e| format!("Failed to read SVG file: {}", e))?;
            Ok(QrImage::Svg(svg_content))
        } else {
            let img =
                image::open(path).map_err(|e| format!("Failed to load raster image: {}", e))?;
            Ok(QrImage::Raster(img))
        }
    }
}
