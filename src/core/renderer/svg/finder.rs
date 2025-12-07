use crate::models::FinderShape;
use std::fmt::Write;

/// Appends the SVG path data for a finder pattern at the specified grid position.
pub fn append_finder_path(data: &mut String, shape: FinderShape, grid_x: f32, grid_y: f32, scale: f32, quiet: f32) {
    let x = (grid_x + quiet) * scale;
    let y = (grid_y + quiet) * scale;
    let size_7 = 7.0 * scale;

    match shape {
        FinderShape::Square => {
            // Outer box (CW)
            let _ = write!(data, "M{x} {y} L{x1} {y} L{x1} {y1} L{x} {y1} Z ", x=x, y=y, x1=x+size_7, y1=y+size_7);
            
            // Inner hole (CCW)
            let size_5 = 5.0 * scale;
            let offset_1 = 1.0 * scale;
            let _ = write!(data, "M{x} {y} L{x} {y1} L{x1} {y1} L{x1} {y} Z ", x=x+offset_1, y=y+offset_1, x1=x+offset_1+size_5, y1=y+offset_1+size_5);

            // Center box (CW)
            let size_3 = 3.0 * scale;
            let offset_2 = 2.0 * scale;
            let _ = write!(data, "M{x} {y} L{x1} {y} L{x1} {y1} L{x} {y1} Z ", x=x+offset_2, y=y+offset_2, x1=x+offset_2+size_3, y1=y+offset_2+size_3);
        },
        FinderShape::Circle => {
            let cx = x + size_7 / 2.0;
            let cy = y + size_7 / 2.0;
            
            // Outer Circle (CW)
            let r7 = size_7 / 2.0;
            let _ = write!(data, "M{mx} {my} A{r} {r} 0 1 1 {ax1} {ay1} A{r} {r} 0 1 1 {mx} {my} ",
                mx=cx, my=cy-r7, r=r7, ax1=cx, ay1=cy+r7);

            // Middle Circle (CCW) - Hole
            let r5 = (5.0 * scale) / 2.0;
            let _ = write!(data, "M{mx} {my} A{r} {r} 0 1 0 {ax1} {ay1} A{r} {r} 0 1 0 {mx} {my} ",
                mx=cx, my=cy-r5, r=r5, ax1=cx, ay1=cy+r5);

            // Inner Circle (CW)
            let r3 = (3.0 * scale) / 2.0;
            let _ = write!(data, "M{mx} {my} A{r} {r} 0 1 1 {ax1} {ay1} A{r} {r} 0 1 1 {mx} {my} ",
                mx=cx, my=cy-r3, r=r3, ax1=cx, ay1=cy+r3);
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
                    let _ = write!(data, "M{} {} L{} {} Q{} {} {} {} L{} {} Q{} {} {} {} L{} {} Q{} {} {} {} L{} {} Q{} {} {} {} Z ",
                        x + r, y,
                        x + s - r, y,
                        x + s, y, x + s, y + r,
                        x + s, y + s - r,
                        x + s, y + s, x + s - r, y + s,
                        x + r, y + s,
                        x, y + s, x, y + s - r,
                        x, y + r,
                        x, y, x + r, y
                    );
                } else {
                    let _ = write!(data, "M{} {} Q{} {} {} {} L{} {} Q{} {} {} {} L{} {} Q{} {} {} {} L{} {} Q{} {} {} {} L{} {} Z ",
                        x + r, y,
                        x, y, x, y + r,
                        x, y + s - r,
                        x, y + s, x + r, y + s,
                        x + s - r, y + s,
                        x + s, y + s, x + s, y + s - r,
                        x + s, y + r,
                        x + s, y, x + s - r, y,
                        x + r, y
                    );
                }
            };
            
            append_rounded_rect(x, y, size_7, r_outer, true);
            append_rounded_rect(x + offset_1, y + offset_1, size_5, r_inner, false);
            append_rounded_rect(x + offset_2, y + offset_2, size_3, r_center, true);
        }
    }
}
