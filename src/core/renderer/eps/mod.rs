use crate::core::renderer::{QrGrid, QrRenderer, utils};
use crate::models::{GradientDirection, QrConfig, QrImage};
use std::fmt::Write;

mod finder;
mod module;

use finder::append_finder_path;
use module::append_module_path;

/// Renders the QR code grid into an EPS string based on the provided configuration.
pub fn render_eps<G: QrGrid + ?Sized>(
    grid: &G,
    options: &QrConfig,
    pixel_size: f32,
) -> Result<String, String> {
    let size = grid.size();
    let quiet_zone = options.quiet_zone as f32;
    let width_modules = size as f32 + quiet_zone * 2.0;
    let width_px = width_modules * pixel_size;
    let height_px = width_px; // QR codes are square

    let mut eps = String::new();

    writeln!(&mut eps, "%!PS-Adobe-3.0 EPSF-3.0").unwrap();
    writeln!(&mut eps, "%%BoundingBox: 0 0 {} {}", width_px, height_px).unwrap();
    writeln!(&mut eps, "%%Creator: Qrosity").unwrap();
    writeln!(&mut eps, "%%Title: QR Code").unwrap();
    writeln!(&mut eps, "%%EndComments").unwrap();

    writeln!(&mut eps, "0 {} translate", height_px).unwrap();
    writeln!(&mut eps, "1 -1 scale").unwrap();

    if let Some(bg_color) = parse_color(&options.background) {
        writeln!(
            &mut eps,
            "newpath 0 0 moveto {} 0 lineto {} {} lineto 0 {} lineto closepath",
            width_px, width_px, height_px, height_px
        )
        .unwrap();
        writeln!(
            &mut eps,
            "{} {} {} setrgbcolor fill",
            bg_color.0, bg_color.1, bg_color.2
        )
        .unwrap();
    }

    let mut path_data = String::new();

    for y in 0..size {
        for x in 0..size {
            if grid.is_dark(x, y) {
                if grid.is_finder(x, y) {
                    continue; // Finders are drawn separately
                }

                let px = (x as f32 + quiet_zone) * pixel_size;
                let py = (y as f32 + quiet_zone) * pixel_size;
                let ctx = grid.module_context(x, y);

                append_module_path(&mut path_data, options.shape, px, py, pixel_size, &ctx);
            }
        }
    }

    // Finders
    // Top-Left
    append_finder_path(
        &mut path_data,
        options.finder,
        quiet_zone * pixel_size,
        quiet_zone * pixel_size,
        pixel_size,
    );
    // Top-Right
    append_finder_path(
        &mut path_data,
        options.finder,
        (size as f32 - 7.0 + quiet_zone) * pixel_size,
        quiet_zone * pixel_size,
        pixel_size,
    );
    // Bottom-Left
    append_finder_path(
        &mut path_data,
        options.finder,
        quiet_zone * pixel_size,
        (size as f32 - 7.0 + quiet_zone) * pixel_size,
        pixel_size,
    );

    // Fill Foreground
    if !path_data.is_empty() {
        writeln!(&mut eps, "gsave").unwrap();
        writeln!(&mut eps, "newpath").unwrap();
        eps.push_str(&path_data);

        // Color handling
        if options.foreground.len() > 1 {
            // Gradient
            // We need to define a clipping path from the QR shape and then draw the gradient
            writeln!(&mut eps, "clip").unwrap();

            let colors: Vec<(f32, f32, f32)> = options
                .foreground
                .iter()
                .filter_map(|c| parse_color(c))
                .collect();

            if !colors.is_empty() {
                let (x0, y0, r0, x1, y1, r1) =
                    utils::get_gradient_coords(options.gradient_direction, width_px, height_px);
                let shading_type = if options.gradient_direction == GradientDirection::Radial {
                    3
                } else {
                    2
                };

                writeln!(&mut eps, "<< /ShadingType {}", shading_type).unwrap();
                writeln!(&mut eps, "   /ColorSpace /DeviceRGB").unwrap();
                if shading_type == 2 {
                    writeln!(
                        &mut eps,
                        "   /Coords [ {:.3} {:.3} {:.3} {:.3} ]",
                        x0, y0, x1, y1
                    )
                    .unwrap();
                } else {
                    writeln!(
                        &mut eps,
                        "   /Coords [ {:.3} {:.3} {:.3} {:.3} {:.3} {:.3} ]",
                        x0, y0, r0, x1, y1, r1
                    )
                    .unwrap();
                }
                writeln!(
                    &mut eps,
                    "   /Function {}",
                    utils::generate_pdf_ps_gradient_function(&colors)
                )
                .unwrap();
                writeln!(&mut eps, "   /Extend [ true true ]").unwrap();
                writeln!(&mut eps, ">> shfill").unwrap();
            }
        } else if let Some(fg_color) = options.foreground.first().and_then(|c| parse_color(c)) {
            writeln!(
                &mut eps,
                "{} {} {} setrgbcolor fill",
                fg_color.0, fg_color.1, fg_color.2
            )
            .unwrap();
        } else {
            // Default black
            writeln!(&mut eps, "0 0 0 setrgbcolor fill").unwrap();
        }
        writeln!(&mut eps, "grestore").unwrap();
    }

    if let Some(image) = &options.image {
        match image {
            crate::models::QrImage::Raster(img) => {
                append_icon(&mut eps, img, size, pixel_size, width_px, height_px);
            }
            crate::models::QrImage::Svg(_) => {
                return Err(format!("EPS renderer does not support SVG icons."));
            }
        }
    } else if let Some(icon_path) = &options.icon {
        QrImage::load_from_path(icon_path).and_then(|image| match image {
            crate::models::QrImage::Raster(img) => {
                append_icon(&mut eps, &img, size, pixel_size, width_px, height_px);
                Ok(())
            }
            crate::models::QrImage::Svg(_) => {
                Err(format!("EPS renderer does not support SVG icons."))
            }
        })?;
    }

    writeln!(&mut eps, "%%EOF").unwrap();

    Ok(eps)
}

fn append_icon(
    eps: &mut String,
    img: &image::DynamicImage,
    size: usize,
    pixel_size: f32,
    width_px: f32,
    height_px: f32,
) {
    let img = img.to_rgba8();
    let (w, h) = img.dimensions();
    let icon_size = (size as f32 * pixel_size) * 0.25;
    let x = (width_px - icon_size) / 2.0;
    let y = (height_px - icon_size) / 2.0;

    writeln!(eps, "gsave").unwrap();
    writeln!(eps, "{} {} translate", x, y).unwrap();
    writeln!(eps, "{} {} scale", icon_size, icon_size).unwrap();

    writeln!(eps, "/picstr {} string def", w * 3).unwrap(); // RGB
    writeln!(eps, "{} {} 8", w, h).unwrap();
    writeln!(eps, "[{} 0 0 {} 0 0]", w, h).unwrap(); // Matrix
    writeln!(eps, "{{ currentfile picstr readhexstring pop }}").unwrap();
    writeln!(eps, "false 3").unwrap(); // false = single source, 3 = RGB
    writeln!(eps, "colorimage").unwrap();

    // Write hex data
    for pixel in img.pixels() {
        // RGB only, ignore alpha for EPS simple implementation
        write!(eps, "{:02x}{:02x}{:02x}", pixel[0], pixel[1], pixel[2]).unwrap();
    }
    writeln!(eps, "").unwrap();

    writeln!(eps, "grestore").unwrap();
}

fn parse_color(hex: &str) -> Option<(f32, f32, f32)> {
    utils::parse_hex_color(hex)
        .map(|(r, g, b)| (r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0))
}

pub struct EpsRenderer {
    data: String,
}

impl EpsRenderer {
    pub fn new(grid: &dyn QrGrid, config: &QrConfig) -> Result<Self, String> {
        let data = render_eps(grid, config, config.ppm as f32)?;
        Ok(Self { data })
    }
}

impl QrRenderer for EpsRenderer {
    fn save(&self, path: &str) -> Result<String, String> {
        let mut path_buf = std::path::PathBuf::from(path);
        if path_buf.extension().is_none() {
            path_buf.set_extension("eps");
        }
        let final_path = path_buf.to_str().ok_or("Invalid path")?.to_string();
        std::fs::write(&final_path, &self.data).map_err(|e| e.to_string())?;
        Ok(final_path)
    }

    fn to_bytes(&self) -> Result<Vec<u8>, String> {
        Ok(self.data.as_bytes().to_vec())
    }
}
