use svg::node::element::path::Data;
use crate::models::FinderShape;

pub fn append_finder_path(data: &mut Data, shape: FinderShape, grid_x: f32, grid_y: f32, scale: f32, quiet: f32) {
    let x = (grid_x + quiet) * scale;
    let y = (grid_y + quiet) * scale;
    let size_7 = 7.0 * scale;

    match shape {
        FinderShape::Square => {
            // Outer box (CW)
            *data = data.clone().move_to((x, y))
                .line_to((x + size_7, y))
                .line_to((x + size_7, y + size_7))
                .line_to((x, y + size_7))
                .close();
            
            // Inner hole (CCW)
            let size_5 = 5.0 * scale;
            let offset_1 = 1.0 * scale;
            *data = data.clone().move_to((x + offset_1, y + offset_1))
                .line_to((x + offset_1, y + offset_1 + size_5))
                .line_to((x + offset_1 + size_5, y + offset_1 + size_5))
                .line_to((x + offset_1 + size_5, y + offset_1))
                .close();

            // Center box (CW)
            let size_3 = 3.0 * scale;
            let offset_2 = 2.0 * scale;
            *data = data.clone().move_to((x + offset_2, y + offset_2))
                .line_to((x + offset_2 + size_3, y + offset_2))
                .line_to((x + offset_2 + size_3, y + offset_2 + size_3))
                .line_to((x + offset_2, y + offset_2 + size_3))
                .close();
        },
        FinderShape::Circle => {
            let cx = x + size_7 / 2.0;
            let cy = y + size_7 / 2.0;
            
            // Outer Circle (CW)
            let r7 = size_7 / 2.0;
            *data = data.clone()
                .move_to((cx, cy - r7))
                .elliptical_arc_to((r7, r7, 0, 1, 1, cx, cy + r7))
                .elliptical_arc_to((r7, r7, 0, 1, 1, cx, cy - r7));

            // Middle Circle (CCW) - Hole
            let r5 = (5.0 * scale) / 2.0;
            *data = data.clone()
                .move_to((cx, cy - r5))
                .elliptical_arc_to((r5, r5, 0, 1, 0, cx, cy + r5))
                .elliptical_arc_to((r5, r5, 0, 1, 0, cx, cy - r5));

            // Inner Circle (CW)
            let r3 = (3.0 * scale) / 2.0;
            *data = data.clone()
                .move_to((cx, cy - r3))
                .elliptical_arc_to((r3, r3, 0, 1, 1, cx, cy + r3))
                .elliptical_arc_to((r3, r3, 0, 1, 1, cx, cy - r3));
        },
        FinderShape::Rounded => {
            let size_5 = 5.0 * scale;
            let offset_1 = 1.0 * scale;
            let size_3 = 3.0 * scale;
            let offset_2 = 2.0 * scale;

            let r_outer = scale * 1.0;
            let r_inner = scale * 0.7;
            let r_center = scale * 0.5;

            let mut append_rounded_rect = |x: f32, y: f32, s: f32, r: f32, cw: bool| {
                let r = r.min(s / 2.0);
                if cw {
                    *data = data.clone()
                        .move_to((x + r, y))
                        .line_to((x + s - r, y))
                        .quadratic_curve_to((x + s, y, x + s, y + r))
                        .line_to((x + s, y + s - r))
                        .quadratic_curve_to((x + s, y + s, x + s - r, y + s))
                        .line_to((x + r, y + s))
                        .quadratic_curve_to((x, y + s, x, y + s - r))
                        .line_to((x, y + r))
                        .quadratic_curve_to((x, y, x + r, y))
                        .close();
                } else {
                    *data = data.clone()
                        .move_to((x + r, y))
                        .quadratic_curve_to((x, y, x, y + r))
                        .line_to((x, y + s - r))
                        .quadratic_curve_to((x, y + s, x + r, y + s))
                        .line_to((x + s - r, y + s))
                        .quadratic_curve_to((x + s, y + s, x + s, y + s - r))
                        .line_to((x + s, y + r))
                        .quadratic_curve_to((x + s, y, x + s - r, y))
                        .line_to((x + r, y))
                        .close();
                }
            };
            append_rounded_rect(x, y, size_7, r_outer, true);
            append_rounded_rect(x + offset_1, y + offset_1, size_5, r_inner, false);
            append_rounded_rect(x + offset_2, y + offset_2, size_3, r_center, true);
        }
    }
}
