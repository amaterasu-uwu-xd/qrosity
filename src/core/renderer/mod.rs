use crate::core::qrgen::QrCode;
use crate::models::{QrConfig, GradientDirection};
use tiny_skia::*;

pub mod module;
pub mod finder;

use module::{draw_module, ModuleContext};
use finder::draw_finder;

// Asumimos que tienes un trait o struct para tu matriz
pub trait QrGrid {
    fn size(&self) -> usize;
    fn get_module(&self, x: usize, y: usize) -> bool;
}

impl QrGrid for QrCode {
    fn size(&self) -> usize {
        self.size() as usize
    }

    fn get_module(&self, x: usize, y: usize) -> bool {
        self.get_module(x as i32, y as i32)
    }
}

pub fn render_qr<G: QrGrid>(
    grid: &G,
    options: &QrConfig,
    pixel_size: f32,
) -> Result<Pixmap, String> {
    let size = grid.size();
    let quiet_zone = options.quiet_zone as f32; // Unidades de modulo
    let width_px = (size as f32 + quiet_zone * 2.0) * pixel_size;

    let mut pixmap = Pixmap::new(width_px as u32, width_px as u32)
        .ok_or("No se pudo crear el buffer de imagen")?;
    let bg_color = parse_color(&options.background)?;
    pixmap.fill(bg_color);

    let mut paint = Paint::default();
    paint.anti_alias = true;

    let mut colors = Vec::new();
    if options.colors.is_empty() {
        colors.push(parse_color("#000000")?);
    } else {
        for color_hex in &options.colors {
            colors.push(parse_color(color_hex)?);
        }
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
            // El radio debe basarse en el tamaño del QR sin la quiet zone para que el gradiente
            // se ajuste mejor al contenido visible.
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
            let is_finder =
                (x < 7 && y < 7) || (x >= size - 7 && y < 7) || (x < 7 && y >= size - 7);

            if is_finder {
                continue;
            }

            if grid.get_module(x, y) {
                let px = (x as f32 + quiet_zone) * pixel_size;
                let py = (y as f32 + quiet_zone) * pixel_size;

                let ctx = ModuleContext {
                    top: y > 0 && grid.get_module(x, y - 1),
                    bottom: y < size - 1 && grid.get_module(x, y + 1),
                    left: x > 0 && grid.get_module(x - 1, y),
                    right: x < size - 1 && grid.get_module(x + 1, y),
                };

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

    // 4. Dibujar los 3 Patrones de Detección (Customizados)
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

    if let Some(icon_path) = &options.icon {
        draw_icon(&mut pixmap, options.ppm, icon_path, size as f32, width_px)?;
    }

    Ok(pixmap)
}

// --- Helper de Color ---
fn parse_color(hex: &str) -> Result<Color, String> {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);

    Ok(Color::from_rgba8(r, g, b, 255))
}

fn draw_icon(pixmap: &mut Pixmap, ppm: u32, path: &str, size: f32, canvas_size: f32) -> Result<(), String> {
    let img_source = image::open(path).map_err(|e| e.to_string())?.to_rgba8();
    let width = img_source.width() as f32;
    let height = img_source.height() as f32;

    let icon_pixmap = Pixmap::from_vec(
        img_source.into_raw(),
        tiny_skia::IntSize::from_wh(width as u32, height as u32).unwrap(),
    )
    .ok_or("Could not create pixmap from icon image")?;

    // Size es el número de módulos del QR, tomamos un 20% 
    
    let target_icon_size = size * 0.2 * ppm as f32;
    let scale = target_icon_size / width.max(height);
    let translate_x = (canvas_size - (width * scale)) / 2.0;
    let translate_y = (canvas_size - (height * scale)) / 2.0;

    let transform = Transform::from_scale(scale, scale).post_translate(translate_x, translate_y);

    let mut paint = PixmapPaint::default();
    paint.blend_mode = BlendMode::SourceOver;

    pixmap.draw_pixmap(0, 0, icon_pixmap.as_ref(), &paint, transform, None);

    Ok(())
}
