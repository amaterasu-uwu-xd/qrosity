use tiny_skia::{PathBuilder, Rect, Pixmap, Paint, FillRule, Transform};
use crate::models::FinderShape;

/// Draws a finder pattern at the specified grid position on the pixmap.
pub fn draw_finder(
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
