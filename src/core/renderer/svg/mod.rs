use crate::models::{QrConfig, GradientDirection};
use crate::core::renderer::{QrGrid, ModuleContext};
use svg::Document;
use svg::node::element::{Path, Rectangle, Definitions, LinearGradient, RadialGradient, Stop, Image};

pub mod module;
pub mod finder;

use module::append_module_path;
use finder::append_finder_path;

pub fn render_svg<G: QrGrid>(
    grid: &G,
    options: &QrConfig,
    pixel_size: f32,
) -> Result<Document, String> {
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

    let mut document = Document::new()
        .set("viewBox", (0, 0, width_px, width_px))
        .set("width", width_px)
        .set("height", width_px);

    // Background
    let bg_rect = Rectangle::new()
        .set("width", "100%")
        .set("height", "100%")
        .set("fill", sanitize_color(&options.background));
    document = document.add(bg_rect);

    // Definitions for Gradients
    let mut defs = Definitions::new();
    let fill_id = "qr-fill";
    let mut use_gradient = false;

    if options.foreground.len() > 1 {
        use_gradient = true;
        let stops: Vec<Stop> = options.foreground.iter().enumerate().map(|(i, color)| {
            let offset = (i as f32 / (options.foreground.len() - 1) as f32) * 100.0;
            Stop::new()
                .set("offset", format!("{}%", offset))
                .set("stop-color", sanitize_color(color))
        }).collect();

        if options.gradient_direction == GradientDirection::Radial {
             let mut gradient = RadialGradient::new()
                .set("id", fill_id)
                .set("cx", "50%")
                .set("cy", "50%")
                .set("r", "70.7%") // approx 1.414 / 2 * 100
                .set("fx", "50%")
                .set("fy", "50%");
            
            for stop in stops {
                gradient = gradient.add(stop);
            }
            defs = defs.add(gradient);
        } else {
            let (x1, y1, x2, y2) = match options.gradient_direction {
                GradientDirection::TopToBottom => ("50%", "0%", "50%", "100%"),
                GradientDirection::LeftToRight => ("0%", "50%", "100%", "50%"),
                GradientDirection::TopLeftToBottomRight => ("0%", "0%", "100%", "100%"),
                GradientDirection::BottomLeftToTopRight => ("0%", "100%", "100%", "0%"),
                _ => ("0%", "0%", "100%", "100%"),
            };

            let mut gradient = LinearGradient::new()
                .set("id", fill_id)
                .set("x1", x1)
                .set("y1", y1)
                .set("x2", x2)
                .set("y2", y2);

            for stop in stops {
                gradient = gradient.add(stop);
            }
            defs = defs.add(gradient);
        }
        document = document.add(defs);
    }

    let fill_attr = if use_gradient {
        format!("url(#{})", fill_id)
    } else {
        sanitize_color(options.foreground.first().unwrap())
    };

    // Draw Modules
    let mut path_data = String::new();

    for y in 0..size {
        for x in 0..size {
            let is_finder = (x < 7 && y < 7) || (x >= size - 7 && y < 7) || (x < 7 && y >= size - 7);
            if is_finder { continue; }

            if grid.get_module(x, y) {
                let px = (x as f32 + quiet_zone) * pixel_size;
                let py = (y as f32 + quiet_zone) * pixel_size;

                let ctx = ModuleContext {
                    top: y > 0 && grid.get_module(x, y - 1),
                    bottom: y < size - 1 && grid.get_module(x, y + 1),
                    left: x > 0 && grid.get_module(x - 1, y),
                    right: x < size - 1 && grid.get_module(x + 1, y),
                };

                append_module_path(&mut path_data, options.shape, px, py, pixel_size, &ctx);
            }
        }
    }

    // Draw Finders
    // Top Left
    append_finder_path(&mut path_data, options.finder, 0.0, 0.0, pixel_size, quiet_zone);
    // Top Right
    append_finder_path(&mut path_data, options.finder, (size - 7) as f32, 0.0, pixel_size, quiet_zone);
    // Bottom Left
    append_finder_path(&mut path_data, options.finder, 0.0, (size - 7) as f32, pixel_size, quiet_zone);

    let path = Path::new()
        .set("fill", fill_attr)
        .set("d", path_data);
    
    document = document.add(path);

    // Icon
    if let Some(icon_path) = &options.icon {
        document = draw_icon(document, icon_path, size, pixel_size, width_px);
    }

    Ok(document)
}

fn draw_icon(
    mut document: Document,
    icon_path: &str,
    size: usize,
    pixel_size: f32,
    width_px: f32,
) -> Document {
    use std::io::Read;
    use base64::prelude::*;

    // Read file content
    let mut file = match std::fs::File::open(icon_path) {
        Ok(f) => f,
        Err(_) => return document,
    };
    let mut buffer = Vec::new();
    if file.read_to_end(&mut buffer).is_ok() {
         let mut width: Option<u32> = None;
         let mut height: Option<u32> = None;
         let mut mime_type = "application/octet-stream";

         // Try to detect SVG
         let is_svg = if let Ok(s) = std::str::from_utf8(&buffer) {
             s.trim_start().starts_with('<') && s.contains("<svg")
         } else {
             false
         };

         if is_svg {
             mime_type = "image/svg+xml";
             if let Ok(s) = std::str::from_utf8(&buffer) {
                 for event in svg::Parser::new(s) {
                     if let svg::parser::Event::Tag("svg", _, attributes) = event {
                         if let Some(w) = attributes.get("width") {
                             if let Ok(val) = w.trim_matches(|c: char| !c.is_numeric() && c != '.').parse::<f32>() {
                                 width = Some(val as u32);
                             }
                         }
                         if let Some(h) = attributes.get("height") {
                             if let Ok(val) = h.trim_matches(|c: char| !c.is_numeric() && c != '.').parse::<f32>() {
                                 height = Some(val as u32);
                             }
                         }
                         if width.is_none() || height.is_none() {
                             if let Some(vb) = attributes.get("viewBox") {
                                 let parts: Vec<f32> = vb.split_whitespace()
                                     .filter_map(|s| s.parse().ok())
                                     .collect();
                                 if parts.len() == 4 {
                                     if width.is_none() { width = Some(parts[2] as u32); }
                                     if height.is_none() { height = Some(parts[3] as u32); }
                                 }
                             }
                         }
                         break;
                     }
                 }
             }
         } else {
             // Try raster
             if let Ok(reader) = image::ImageReader::new(std::io::Cursor::new(&buffer)).with_guessed_format() {
                 if let Ok(dims) = reader.into_dimensions() {
                     width = Some(dims.0);
                     height = Some(dims.1);
                     
                     if let Ok(format) = image::guess_format(&buffer) {
                        mime_type = match format {
                            image::ImageFormat::Png => "image/png",
                            image::ImageFormat::Jpeg => "image/jpeg",
                            image::ImageFormat::Gif => "image/gif",
                            image::ImageFormat::WebP => "image/webp",
                            _ => "application/octet-stream",
                        };
                     }
                 }
             }
         }

         if let (Some(w_px), Some(h_px)) = (width, height) {
                let icon_size = size as f32 * 0.2 * pixel_size;
                let scale = icon_size / (w_px.max(h_px) as f32);
                let w = w_px as f32 * scale;
                let h = h_px as f32 * scale;
                let x = (width_px - w) / 2.0;
                let y = (width_px - h) / 2.0;

                // Encode base64
                let encoded = BASE64_STANDARD.encode(&buffer);
                let href = format!("data:{};base64,{}", mime_type, encoded);

                let image_node = Image::new()
                    .set("x", x)
                    .set("y", y)
                    .set("width", w)
                    .set("height", h)
                    .set("href", href); 
                
                document = document.add(image_node);
         }
    }
    document
}
