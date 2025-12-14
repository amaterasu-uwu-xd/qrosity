use crate::core::renderer::{QrGrid, QrRenderer};
use crate::models::{GradientDirection, QrConfig, QrImage};
use std::fmt::Write;

mod finder;
mod module;

use finder::append_finder_path;
use module::append_module_path;

/// Renders a QR code grid into an SVG String.
pub fn render_svg<G: QrGrid + ?Sized>(
    grid: &G,
    options: &QrConfig,
    pixel_size: f32,
) -> Result<String, String> {
    let size = grid.size();
    let quiet_zone = options.quiet_zone as f32;
    let width_px = (size as f32 + quiet_zone * 2.0) * pixel_size;

    let sanitize_color = |color: &str| -> String {
        if !color.starts_with('#') && color.chars().all(|c| c.is_ascii_hexdigit()) {
            format!("#{}", color)
        } else {
            color.to_string()
        }
    };

    let mut svg = String::new();

    // SVG Header
    writeln!(&mut svg, r#"<svg viewBox="0 0 {w} {h}" width="{w}" height="{h}" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">"#, w=width_px, h=width_px).unwrap();

    // Background
    writeln!(
        &mut svg,
        r#"<rect width="100%" height="100%" fill="{}" />"#,
        sanitize_color(&options.background)
    )
    .unwrap();

    // Definitions for Gradients
    let fill_id = "qr-fill";
    let mut fill_attr =
        sanitize_color(options.foreground.first().unwrap_or(&"#000000".to_string()));

    if options.foreground.len() > 1 {
        fill_attr = format!("url(#{})", fill_id);

        writeln!(&mut svg, "<defs>").unwrap();

        if options.gradient_direction == GradientDirection::Radial {
            writeln!(
                &mut svg,
                r#"<radialGradient id="{}" cx="50%" cy="50%" r="70.7%" fx="50%" fy="50%">"#,
                fill_id
            )
            .unwrap();
        } else {
            let (x1, y1, x2, y2) = match options.gradient_direction {
                GradientDirection::TopToBottom => ("50%", "0%", "50%", "100%"),
                GradientDirection::LeftToRight => ("0%", "50%", "100%", "50%"),
                GradientDirection::TopLeftToBottomRight => ("0%", "0%", "100%", "100%"),
                GradientDirection::BottomLeftToTopRight => ("0%", "100%", "100%", "0%"),
                _ => ("0%", "0%", "100%", "100%"),
            };
            writeln!(
                &mut svg,
                r#"<linearGradient id="{}" x1="{}" y1="{}" x2="{}" y2="{}">"#,
                fill_id, x1, y1, x2, y2
            )
            .unwrap();
        }

        for (i, color) in options.foreground.iter().enumerate() {
            let offset = (i as f32 / (options.foreground.len() - 1) as f32) * 100.0;
            writeln!(
                &mut svg,
                r#"<stop offset="{}%" stop-color="{}" />"#,
                offset,
                sanitize_color(color)
            )
            .unwrap();
        }

        if options.gradient_direction == GradientDirection::Radial {
            writeln!(&mut svg, "</radialGradient>").unwrap();
        } else {
            writeln!(&mut svg, "</linearGradient>").unwrap();
        }
        writeln!(&mut svg, "</defs>").unwrap();
    }

    // Draw Modules
    let mut path_data = String::new();

    for y in 0..size {
        for x in 0..size {
            if grid.is_finder(x, y) {
                continue;
            }

            if grid.is_dark(x, y) {
                let px = (x as f32 + quiet_zone) * pixel_size;
                let py = (y as f32 + quiet_zone) * pixel_size;

                let ctx = grid.module_context(x, y);

                append_module_path(&mut path_data, options.shape, px, py, pixel_size, &ctx);
            }
        }
    }

    // Draw Finders
    // Top Left
    append_finder_path(
        &mut path_data,
        options.finder,
        0.0,
        0.0,
        pixel_size,
        quiet_zone,
    );
    // Top Right
    append_finder_path(
        &mut path_data,
        options.finder,
        (size - 7) as f32,
        0.0,
        pixel_size,
        quiet_zone,
    );
    // Bottom Left
    append_finder_path(
        &mut path_data,
        options.finder,
        0.0,
        (size - 7) as f32,
        pixel_size,
        quiet_zone,
    );

    if !path_data.is_empty() {
        writeln!(
            &mut svg,
            r#"<path fill="{}" d="{}" />"#,
            fill_attr, path_data
        )
        .unwrap();
    }

    if let Some(image) = &options.image {
        append_icon(&mut svg, image, size, pixel_size, width_px)?;
    } else if let Some(icon_path) = &options.icon {
        match QrImage::load_from_path(icon_path) {
            Ok(image) => {
                append_icon(&mut svg, &image, size, pixel_size, width_px)?;
            }
            Err(e) => {
                return Err(format!("Failed to load icon image: {}", e));
            }
        }
    }

    writeln!(&mut svg, "</svg>").unwrap();

    Ok(svg)
}

fn append_icon(
    svg: &mut String,
    image: &crate::models::QrImage,
    size: usize,
    pixel_size: f32,
    width_px: f32,
) -> Result<(), String> {
    let mut width: Option<u32> = None;
    let mut height: Option<u32> = None;
    let mime_type;
    let mut encoded_data = String::new();

    match image {
        crate::models::QrImage::Svg(content) => {
            mime_type = "image/svg+xml";
            encoded_data = encode_base64(content.as_bytes());

            // Simple manual parsing for width/height/viewBox
            if let Some(start) = content.find("<svg") {
                let end = content[start..].find('>').unwrap_or(content.len()) + start;
                let tag = &content[start..end];

                let parse_attr = |attr: &str| -> Option<u32> {
                    if let Some(pos) = tag.find(attr) {
                        let rest = &tag[pos + attr.len()..];
                        if let Some(quote_start) = rest.find('"') {
                            let rest = &rest[quote_start + 1..];
                            if let Some(quote_end) = rest.find('"') {
                                let val_str = &rest[..quote_end];
                                let num_str: String = val_str
                                    .chars()
                                    .filter(|c| c.is_numeric() || *c == '.')
                                    .collect();
                                return num_str.parse::<f32>().ok().map(|v| v as u32);
                            }
                        }
                    }
                    None
                };

                width = parse_attr("width=");
                height = parse_attr("height=");

                if width.is_none() || height.is_none() {
                    if let Some(pos) = tag.find("viewBox=") {
                        let rest = &tag[pos + 8..];
                        if let Some(quote_start) = rest.find('"') {
                            let rest = &rest[quote_start + 1..];
                            if let Some(quote_end) = rest.find('"') {
                                let val_str = &rest[..quote_end];
                                let parts: Vec<f32> = val_str
                                    .split_whitespace()
                                    .filter_map(|s| s.parse().ok())
                                    .collect();
                                if parts.len() == 4 {
                                    if width.is_none() {
                                        width = Some(parts[2] as u32);
                                    }
                                    if height.is_none() {
                                        height = Some(parts[3] as u32);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        crate::models::QrImage::Raster(img) => {
            width = Some(img.width());
            height = Some(img.height());
            mime_type = "image/png";

            let mut buffer = Vec::new();
            if img
                .write_to(
                    &mut std::io::Cursor::new(&mut buffer),
                    image::ImageFormat::Png,
                )
                .is_ok()
            {
                encoded_data = encode_base64(&buffer);
            }
        }
    }

    if let (Some(w_px), Some(h_px)) = (width, height) {
        if encoded_data.is_empty() {
            return Err(format!("Failed to encode icon image data."))?;
        }

        let icon_size = size as f32 * 0.25 * pixel_size;
        let scale = icon_size / (w_px.max(h_px) as f32);
        let w = w_px as f32 * scale;
        let h = h_px as f32 * scale;
        let x = (width_px - w) / 2.0;
        let y = (width_px - h) / 2.0;

        let href = format!("data:{};base64,{}", mime_type, encoded_data);

        writeln!(
            svg,
            r#"<image x="{}" y="{}" width="{}" height="{}" href="{}" />"#,
            x, y, w, h, href
        )
        .unwrap();
    } else {
        return Err(format!("Icon image dimensions could not be determined."))?;
    }
    Ok(())
}

fn encode_base64(input: &[u8]) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut output = String::with_capacity((input.len() * 4 / 3) + 4);
    let mut i = 0;
    let len = input.len();

    while i < len {
        let b0 = input[i];
        let b1 = if i + 1 < len { input[i + 1] } else { 0 };
        let b2 = if i + 2 < len { input[i + 2] } else { 0 };

        let idx0 = (b0 >> 2) as usize;
        let idx1 = (((b0 & 0x03) << 4) | (b1 >> 4)) as usize;
        let idx2 = (((b1 & 0x0F) << 2) | (b2 >> 6)) as usize;
        let idx3 = (b2 & 0x3F) as usize;

        output.push(ALPHABET[idx0] as char);
        output.push(ALPHABET[idx1] as char);

        if i + 1 < len {
            output.push(ALPHABET[idx2] as char);
        } else {
            output.push('=');
        }

        if i + 2 < len {
            output.push(ALPHABET[idx3] as char);
        } else {
            output.push('=');
        }

        i += 3;
    }
    output
}

pub struct SvgRenderer {
    data: String,
}

impl SvgRenderer {
    pub fn new(grid: &dyn QrGrid, config: &QrConfig) -> Result<Self, String> {
        let data = render_svg(grid, config, config.ppm as f32)?;
        Ok(Self { data })
    }
}

impl QrRenderer for SvgRenderer {
    fn save(&self, path: &str) -> Result<String, String> {
        let mut path_buf = std::path::PathBuf::from(path);
        if path_buf.extension().is_none() {
            path_buf.set_extension("svg");
        }
        let final_path = path_buf.to_str().ok_or("Invalid path")?.to_string();
        std::fs::write(&final_path, &self.data).map_err(|e| e.to_string())?;
        Ok(final_path)
    }

    fn to_bytes(&self) -> Result<Vec<u8>, String> {
        Ok(self.data.as_bytes().to_vec())
    }
}
