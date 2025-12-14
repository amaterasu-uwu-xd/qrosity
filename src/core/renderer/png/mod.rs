use crate::core::renderer::{QrGrid, QrRenderer, utils};
use crate::models::{GradientDirection, OutputFormat, QrConfig, QrImage};
use tiny_skia::*;

mod finder;
mod module;

use finder::draw_finder;
use module::draw_module;

/// Renders a QR code grid into a PNG image represented as a Pixmap.
/// If failed, returns an error message as a String.
pub fn render_qr<G: QrGrid + ?Sized>(
    grid: &G,
    options: &QrConfig,
    pixel_size: f32,
) -> Result<Pixmap, String> {
    let size = grid.size();
    let quiet_zone = options.quiet_zone as f32; // Unidades de modulo
    let width_px = (size as f32 + quiet_zone * 2.0) * pixel_size;

    let mut pixmap =
        Pixmap::new(width_px as u32, width_px as u32).ok_or("Error creating image buffer")?;
    let bg_color = parse_color(&options.background)?;
    pixmap.fill(bg_color);

    let mut paint = Paint::default();
    paint.anti_alias = true;

    let mut colors = Vec::new();
    for color_hex in &options.foreground {
        colors.push(parse_color(color_hex)?);
    }
    if colors.len() > 1 {
        let stops: Vec<GradientStop> = colors
            .iter()
            .enumerate()
            .map(|(i, &color)| {
                let pos = i as f32 / (colors.len() - 1) as f32;
                GradientStop::new(pos, color)
            })
            .collect();

        let shader = if options.gradient_direction == GradientDirection::Radial {
            let center_x = width_px / 2.0;
            let center_y = width_px / 2.0;
            let qr_size_px = size as f32 * pixel_size;
            let radius = qr_size_px / 2.0 * 1.414;
            RadialGradient::new(
                Point::from_xy(center_x, center_y),
                Point::from_xy(center_x, center_y),
                radius,
                stops,
                SpreadMode::Pad,
                Transform::identity(),
            )
            .ok_or("Failed to create radial gradient")?
        } else {
            let (start_point, end_point) = match options.gradient_direction {
                GradientDirection::TopToBottom => (
                    Point::from_xy(width_px / 2.0, quiet_zone * pixel_size),
                    Point::from_xy(width_px / 2.0, width_px - quiet_zone * pixel_size),
                ),
                GradientDirection::LeftToRight => (
                    Point::from_xy(quiet_zone * pixel_size, width_px / 2.0),
                    Point::from_xy(width_px - quiet_zone * pixel_size, width_px / 2.0),
                ),
                GradientDirection::TopLeftToBottomRight => (
                    Point::from_xy(quiet_zone * pixel_size, quiet_zone * pixel_size),
                    Point::from_xy(
                        width_px - quiet_zone * pixel_size,
                        width_px - quiet_zone * pixel_size,
                    ),
                ),
                GradientDirection::BottomLeftToTopRight => (
                    Point::from_xy(quiet_zone * pixel_size, width_px - quiet_zone * pixel_size),
                    Point::from_xy(width_px - quiet_zone * pixel_size, quiet_zone * pixel_size),
                ),
                _ => unreachable!(),
            };

            LinearGradient::new(
                start_point,
                end_point,
                stops,
                SpreadMode::Pad,
                Transform::identity(),
            )
            .ok_or("Failed to create linear gradient")?
        };

        paint.shader = shader;
    } else {
        paint.set_color(colors[0]);
    }

    for y in 0..size {
        for x in 0..size {
            if grid.is_finder(x, y) {
                continue;
            }

            if grid.is_dark(x, y) {
                let px = (x as f32 + quiet_zone) * pixel_size;
                let py = (y as f32 + quiet_zone) * pixel_size;

                let ctx = grid.module_context(x, y);

                let path = draw_module(options.shape, px, py, pixel_size, &ctx);
                pixmap.fill_path(
                    &path,
                    &paint,
                    FillRule::Winding,
                    Transform::identity(),
                    None,
                );
            }
        }
    }

    draw_finder(
        &mut pixmap,
        0.0,
        0.0,
        pixel_size,
        quiet_zone,
        options.finder,
        &paint,
    );
    draw_finder(
        &mut pixmap,
        (size - 7) as f32,
        0.0,
        pixel_size,
        quiet_zone,
        options.finder,
        &paint,
    );
    draw_finder(
        &mut pixmap,
        0.0,
        (size - 7) as f32,
        pixel_size,
        quiet_zone,
        options.finder,
        &paint,
    );

    if let Some(image) = &options.image {
        match image {
            crate::models::QrImage::Raster(img) => {
                draw_icon(&mut pixmap, options.ppm, img, size as f32, width_px)?;
            }
            crate::models::QrImage::Svg(_) => {
                return Err(format!(
                    "{} format does not support SVG icons.",
                    options.format
                ));
            }
        }
    } else if let Some(icon_path) = &options.icon {
        match QrImage::load_from_path(icon_path) {
            Ok(img) => match img {
                crate::models::QrImage::Raster(img) => {
                    draw_icon(&mut pixmap, options.ppm, &img, size as f32, width_px)?;
                }
                crate::models::QrImage::Svg(_) => {
                    return Err(format!(
                        "{} format does not support SVG icons.",
                        options.format
                    ));
                }
            },
            Err(e) => Err(e)?,
        }
    }

    Ok(pixmap)
}

/// Parses a hex color string (e.g., "#RRGGBB" or "RRGGBB") into a Color.
fn parse_color(hex: &str) -> Result<Color, String> {
    let (r, g, b) = utils::parse_hex_color(hex).ok_or_else(|| format!("Invalid color: {}", hex))?;
    Ok(Color::from_rgba8(r, g, b, 255))
}

/// Draws an icon at the center of the QR code pixmap.
/// The icon is scaled to fit within 20% of the QR code size ignoring quiet zones.
fn draw_icon(
    pixmap: &mut Pixmap,
    ppm: u32,
    img: &image::DynamicImage,
    size: f32,
    canvas_size: f32,
) -> Result<(), String> {
    let mut img_source = img.to_rgba8();

    // Premultiply alpha for tiny-skia
    for pixel in img_source.pixels_mut() {
        let alpha = pixel[3] as f32 / 255.0;
        pixel[0] = (pixel[0] as f32 * alpha + 0.5) as u8;
        pixel[1] = (pixel[1] as f32 * alpha + 0.5) as u8;
        pixel[2] = (pixel[2] as f32 * alpha + 0.5) as u8;
    }

    let width = img_source.width() as f32;
    let height = img_source.height() as f32;

    let icon_pixmap = Pixmap::from_vec(
        img_source.into_raw(),
        tiny_skia::IntSize::from_wh(width as u32, height as u32).unwrap(),
    )
    .ok_or("Could not create pixmap from icon image")?;

    let target_icon_size = size * 0.25 * ppm as f32;
    let scale = target_icon_size / width.max(height);
    let translate_x = (canvas_size - (width * scale)) / 2.0;
    let translate_y = (canvas_size - (height * scale)) / 2.0;

    let transform = Transform::from_scale(scale, scale).post_translate(translate_x, translate_y);

    let mut paint = PixmapPaint::default();
    paint.blend_mode = BlendMode::SourceOver;

    pixmap.draw_pixmap(0, 0, icon_pixmap.as_ref(), &paint, transform, None);

    Ok(())
}

/// Saves the rendered Pixmap to a file.
/// Supports PNG natively via tiny-skia, and other formats (JPG, BMP, etc.) via the image crate.
pub fn save_image(pixmap: &Pixmap, path: &str, format: OutputFormat) -> Result<String, String> {
    let mut path_buf = std::path::PathBuf::from(path);
    if path_buf.extension().is_none() {
        let ext = match format {
            OutputFormat::Png => "png",
            OutputFormat::Jpg | OutputFormat::Jpeg => "jpg",
            OutputFormat::Bmp => "bmp",
            OutputFormat::Tiff => "tiff",
            OutputFormat::Gif => "gif",
            OutputFormat::Ico => "ico",
            OutputFormat::Webp => "webp",
            _ => "png",
        };
        path_buf.set_extension(ext);
    }
    let final_path = path_buf.to_str().ok_or("Invalid path")?.to_string();

    if format == OutputFormat::Png {
        pixmap
            .save_png(&final_path)
            .map_err(|e| format!("Error saving PNG: {}", e))?;
    } else {
        let width = pixmap.width();
        let height = pixmap.height();

        // Demultiply alpha since tiny-skia uses premultiplied alpha
        let data: Vec<u8> = pixmap
            .pixels()
            .iter()
            .flat_map(|p| {
                let c = p.demultiply();
                [c.red(), c.green(), c.blue(), c.alpha()]
            })
            .collect();

        let img = image::RgbaImage::from_raw(width, height, data)
            .ok_or("Error creating image buffer from pixmap")?;

        let dynamic_image = image::DynamicImage::ImageRgba8(img);

        // Handle formats that don't support alpha or need specific conversion
        let output_image = match format {
            OutputFormat::Jpg | OutputFormat::Jpeg | OutputFormat::Bmp => {
                image::DynamicImage::ImageRgb8(dynamic_image.into_rgb8())
            }
            _ => dynamic_image,
        };

        let image_format = match format {
            OutputFormat::Jpg | OutputFormat::Jpeg => image::ImageFormat::Jpeg,
            OutputFormat::Bmp => image::ImageFormat::Bmp,
            OutputFormat::Tiff => image::ImageFormat::Tiff,
            OutputFormat::Gif => image::ImageFormat::Gif,
            OutputFormat::Ico => image::ImageFormat::Ico,
            OutputFormat::Webp => image::ImageFormat::WebP,
            OutputFormat::Png => image::ImageFormat::Png,
            _ => return Err(format!("Unsupported format: {:?}", format)),
        };

        output_image
            .save_with_format(&final_path, image_format)
            .map_err(|e| format!("Error saving image: {}", e))?;
    }
    Ok(final_path)
}

pub struct PngRenderer {
    pixmap: Pixmap,
    format: OutputFormat,
}

impl PngRenderer {
    pub fn new(grid: &dyn QrGrid, config: &QrConfig) -> Result<Self, String> {
        let pixmap = render_qr(grid, config, config.ppm as f32)?;
        Ok(Self {
            pixmap,
            format: config.format,
        })
    }
}

impl QrRenderer for PngRenderer {
    fn save(&self, path: &str) -> Result<String, String> {
        save_image(&self.pixmap, path, self.format)
    }

    fn to_bytes(&self) -> Result<Vec<u8>, String> {
        if self.format == OutputFormat::Png {
            self.pixmap
                .encode_png()
                .map_err(|e| format!("Error encoding PNG: {}", e))
        } else {
            let width = self.pixmap.width();
            let height = self.pixmap.height();

            // Demultiply alpha since tiny-skia uses premultiplied alpha
            let data: Vec<u8> = self
                .pixmap
                .pixels()
                .iter()
                .flat_map(|p| {
                    let c = p.demultiply();
                    [c.red(), c.green(), c.blue(), c.alpha()]
                })
                .collect();

            let img = image::RgbaImage::from_raw(width, height, data)
                .ok_or("Error creating image buffer from pixmap")?;

            let dynamic_image = image::DynamicImage::ImageRgba8(img);

            // Handle formats that don't support alpha or need specific conversion
            let output_image = match self.format {
                OutputFormat::Jpg | OutputFormat::Jpeg | OutputFormat::Bmp => {
                    image::DynamicImage::ImageRgb8(dynamic_image.into_rgb8())
                }
                _ => dynamic_image,
            };

            let image_format = match self.format {
                OutputFormat::Jpg | OutputFormat::Jpeg => image::ImageFormat::Jpeg,
                OutputFormat::Bmp => image::ImageFormat::Bmp,
                OutputFormat::Tiff => image::ImageFormat::Tiff,
                OutputFormat::Gif => image::ImageFormat::Gif,
                OutputFormat::Ico => image::ImageFormat::Ico,
                OutputFormat::Webp => image::ImageFormat::WebP,
                OutputFormat::Png => image::ImageFormat::Png,
                _ => return Err(format!("Unsupported format: {:?}", self.format)),
            };

            let mut bytes: Vec<u8> = Vec::new();
            output_image
                .write_to(&mut std::io::Cursor::new(&mut bytes), image_format)
                .map_err(|e| format!("Error encoding image: {}", e))?;
            Ok(bytes)
        }
    }
}
