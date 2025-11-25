use crate::core::qrgen::QrCode;
use crate::models::config::{ FinderShape, ModuleShape, QrConfig};
use tiny_skia::*;

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

    let fg_color = parse_color(&options.foreground)?;
    let mut paint = Paint::default();
    paint.set_color(fg_color);
    paint.anti_alias = true;

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

                let path = draw_module(options.shape, px, py, pixel_size);
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

// --- Helpers de Formas ---

fn draw_module(shape: ModuleShape, x: f32, y: f32, size: f32) -> Path {
    let mut pb = PathBuilder::new();
    match shape {
        ModuleShape::Square => {
            pb.push_rect(Rect::from_xywh(x, y, size, size).unwrap());
        }
        ModuleShape::Dots => {
            let radius = size / 2.0 - (size * 0.05);
            pb.push_circle(x + radius, y + radius, radius);
        }
        ModuleShape::Gapped => {
            let s = size - (size * 0.1);
            let r = size * 0.1; // Radio de las esquinas redondeadas
            pb.move_to(x + r, y);
            pb.line_to(x + s - r, y);
            pb.quad_to(x + s, y, x + s, y + r);
            pb.line_to(x + s, y + s - r);
            pb.quad_to(x + s, y + s, x + s - r, y + s);
            pb.line_to(x + r, y + s);
            pb.quad_to(x, y + s, x, y + s - r);
            pb.line_to(x, y + r);
            pb.quad_to(x, y, x + r, y);
            pb.close();
        }
        ModuleShape::Diamond => {
            pb.move_to(x + size / 2.0, y);
            pb.line_to(x + size, y + size / 2.0);
            pb.line_to(x + size / 2.0, y + size);
            pb.line_to(x, y + size / 2.0);
            pb.close();
        }
        ModuleShape::Heart => {
            let s = size;
            let s_half = s / 2.0;
            pb.move_to(x + s_half, y + s * 0.3);
            pb.cubic_to(x + s_half, y, x + s * 0.95, y, x + s * 0.95, y + s * 0.3);

            pb.cubic_to(
                x + s * 0.95,
                y + s * 0.6,
                x + s * 0.65,
                y + s * 0.9,
                x + s_half,
                y + s,
            );

            pb.cubic_to(
                x + s * 0.35,
                y + s * 0.9,
                x + s * 0.05,
                y + s * 0.6,
                x + s * 0.05,
                y + s * 0.3,
            );

            pb.cubic_to(x + s * 0.05, y, x + s_half, y, x + s_half, y + s * 0.3);

            pb.close();
        }
    }
    pb.finish().unwrap()
}

fn draw_finder(
    pixmap: &mut Pixmap,
    grid_x: f32,
    grid_y: f32,
    scale: f32,
    quiet: f32,
    shape: FinderShape,
    paint: &Paint,
) {
    let x = (grid_x + quiet) * scale;
    let y = (grid_y + quiet) * scale;
    let size_7 = 7.0 * scale;

    let mut pb = PathBuilder::new();

    match shape {
        FinderShape::Square => {
            // 1. Exterior (7x7)
            pb.push_rect(Rect::from_xywh(x, y, size_7, size_7).unwrap());
            // 2. Hueco (5x5) - Nótese la dirección o simplemente usar EvenOdd
            let size_5 = 5.0 * scale;
            let offset_1 = 1.0 * scale;
            pb.push_rect(Rect::from_xywh(x + offset_1, y + offset_1, size_5, size_5).unwrap());
            // 3. Centro (3x3)
            let size_3 = 3.0 * scale;
            let offset_2 = 2.0 * scale;
            pb.push_rect(Rect::from_xywh(x + offset_2, y + offset_2, size_3, size_3).unwrap());
        }
        FinderShape::Circle => {
            let center_x = x + size_7 / 2.0;
            let center_y = y + size_7 / 2.0;

            pb.push_circle(center_x, center_y, size_7 / 2.0);
            pb.push_circle(center_x, center_y, (5.0 * scale) / 2.0);
            pb.push_circle(center_x, center_y, (3.0 * scale) / 2.0);
        }
        FinderShape::Rounded => {
            let x_origin = (grid_x + quiet) * scale;
            let y_origin = (grid_y + quiet) * scale;
            let size_7 = 7.0 * scale;

            // 1. Exterior (7x7)
            let r_outer = scale * 1.0;
            let size_5 = 5.0 * scale;
            let offset_1 = 1.0 * scale;
            let r_inner = scale * 0.7;
            let size_3 = 3.0 * scale;
            let offset_2 = 2.0 * scale;
            let r_center = scale * 0.5;

            let mut draw_rounded_rect = |x_start: f32, y_start: f32, s: f32, r: f32| {
                let r_safe = r.min(s / 2.0);
                pb.move_to(x_start + r_safe, y_start);
                pb.line_to(x_start + s - r_safe, y_start);
                pb.quad_to(x_start + s, y_start, x_start + s, y_start + r_safe);
                pb.line_to(x_start + s, y_start + s - r_safe);
                pb.quad_to(x_start + s, y_start + s, x_start + s - r_safe, y_start + s);
                pb.line_to(x_start + r_safe, y_start + s);
                pb.quad_to(x_start, y_start + s, x_start, y_start + s - r_safe);
                pb.line_to(x_start, y_start + r_safe);
                pb.quad_to(x_start, y_start, x_start + r_safe, y_start);
            };
            draw_rounded_rect(x_origin, y_origin, size_7, r_outer);
            draw_rounded_rect(x_origin + offset_1, y_origin + offset_1, size_5, r_inner);
            draw_rounded_rect(x_origin + offset_2, y_origin + offset_2, size_3, r_center);
            pb.close();
        }
    }

    let path = pb.finish().unwrap();
    // Usamos EvenOdd para que los rectángulos anidados creen huecos automáticamente
    let stroke_paint = paint.clone();
    // Truco: Para "recortar" con EvenOdd, dibujamos todo con el mismo color
    pixmap.fill_path(
        &path,
        &stroke_paint,
        FillRule::EvenOdd,
        Transform::identity(),
        None,
    );
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
