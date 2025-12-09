use crate::models::FinderShape;
use std::fmt::Write;

const K: f32 = 0.5522847498;

pub fn append_finder_path(data: &mut String, shape: FinderShape, x: f32, y: f32, size: f32) {
    let module_count = 7.0;
    let total_size = size * module_count;
    
    match shape {
        FinderShape::Square => {
            // Outer 7x7 (CW)
            writeln!(data, "{:.4} {:.4} {:.4} {:.4} re", x, y, total_size, total_size).unwrap();
            
            // Inner 5x5 (Hole) - Must be CCW for Non-Zero winding rule
            let inner_start = size;
            let inner_size = size * 5.0;
            let ix = x + inner_start;
            let iy = y + inner_start;
            // Draw CCW rect: Top-Left -> Bottom-Left -> Bottom-Right -> Top-Right
            writeln!(data, "{:.4} {:.4} m", ix, iy).unwrap();
            writeln!(data, "{:.4} {:.4} l", ix, iy + inner_size).unwrap();
            writeln!(data, "{:.4} {:.4} l", ix + inner_size, iy + inner_size).unwrap();
            writeln!(data, "{:.4} {:.4} l", ix + inner_size, iy).unwrap();
            writeln!(data, "h").unwrap();
            
            // Center 3x3 (CW)
            let center_start = size * 2.0;
            let center_size = size * 3.0;
            writeln!(data, "{:.4} {:.4} {:.4} {:.4} re", x + center_start, y + center_start, center_size, center_size).unwrap();
        }
        FinderShape::Circle => {
             let cx = x + total_size / 2.0;
             let cy = y + total_size / 2.0;
             
             // Outer circle (CW)
             draw_circle(data, cx, cy, total_size / 2.0);
             // Inner hole (CCW)
             draw_circle_ccw(data, cx, cy, size * 2.5);
             // Center circle (CW)
             draw_circle(data, cx, cy, size * 1.5);
        }
        FinderShape::Rounded => {
            let r_outer = size; // 1.0 module
            
            // Outer rounded rect (CW)
            draw_rounded_rect(data, x, y, total_size, total_size, r_outer);
            
            // Inner hole (CCW)
            let inner_start = size;
            let inner_size = size * 5.0;
            let r_inner = size * 0.7; // Slightly tighter radius for inner
            draw_rounded_rect_ccw(data, x + inner_start, y + inner_start, inner_size, inner_size, r_inner);
                
            // Center rounded rect (CW)
            let center_start = size * 2.0;
            let center_size = size * 3.0;
            let r_center = size * 0.5;
            draw_rounded_rect(data, x + center_start, y + center_start, center_size, center_size, r_center);
        }
    }
}

fn draw_circle(data: &mut String, cx: f32, cy: f32, r: f32) {
    // Clockwise
    writeln!(data, "{:.4} {:.4} m", cx + r, cy).unwrap();
    writeln!(data, "{:.4} {:.4} {:.4} {:.4} {:.4} {:.4} c", cx + r, cy + r * K, cx + r * K, cy + r, cx, cy + r).unwrap();
    writeln!(data, "{:.4} {:.4} {:.4} {:.4} {:.4} {:.4} c", cx - r * K, cy + r, cx - r, cy + r * K, cx - r, cy).unwrap();
    writeln!(data, "{:.4} {:.4} {:.4} {:.4} {:.4} {:.4} c", cx - r, cy - r * K, cx - r * K, cy - r, cx, cy - r).unwrap();
    writeln!(data, "{:.4} {:.4} {:.4} {:.4} {:.4} {:.4} c", cx + r * K, cy - r, cx + r, cy - r * K, cx + r, cy).unwrap();
    writeln!(data, "h").unwrap();
}

fn draw_circle_ccw(data: &mut String, cx: f32, cy: f32, r: f32) {
    // Counter-Clockwise
    writeln!(data, "{:.4} {:.4} m", cx + r, cy).unwrap();
    writeln!(data, "{:.4} {:.4} {:.4} {:.4} {:.4} {:.4} c", cx + r, cy - r * K, cx + r * K, cy - r, cx, cy - r).unwrap();
    writeln!(data, "{:.4} {:.4} {:.4} {:.4} {:.4} {:.4} c", cx - r * K, cy - r, cx - r, cy - r * K, cx - r, cy).unwrap();
    writeln!(data, "{:.4} {:.4} {:.4} {:.4} {:.4} {:.4} c", cx - r, cy + r * K, cx - r * K, cy + r, cx, cy + r).unwrap();
    writeln!(data, "{:.4} {:.4} {:.4} {:.4} {:.4} {:.4} c", cx + r * K, cy + r, cx + r, cy + r * K, cx + r, cy).unwrap();
    writeln!(data, "h").unwrap();
}

fn draw_rounded_rect(data: &mut String, x: f32, y: f32, w: f32, h: f32, r: f32) {
    // Clockwise
    let x2 = x + w;
    let y2 = y + h;
    
    writeln!(data, "{:.4} {:.4} m", x + r, y).unwrap();
    writeln!(data, "{:.4} {:.4} l", x2 - r, y).unwrap();
    writeln!(data, "{:.4} {:.4} {:.4} {:.4} {:.4} {:.4} c", x2 - r + r * K, y, x2, y + r - r * K, x2, y + r).unwrap();
    writeln!(data, "{:.4} {:.4} l", x2, y2 - r).unwrap();
    writeln!(data, "{:.4} {:.4} {:.4} {:.4} {:.4} {:.4} c", x2, y2 - r + r * K, x2 - r + r * K, y2, x2 - r, y2).unwrap();
    writeln!(data, "{:.4} {:.4} l", x + r, y2).unwrap();
    writeln!(data, "{:.4} {:.4} {:.4} {:.4} {:.4} {:.4} c", x + r - r * K, y2, x, y2 - r + r * K, x, y2 - r).unwrap();
    writeln!(data, "{:.4} {:.4} l", x, y + r).unwrap();
    writeln!(data, "{:.4} {:.4} {:.4} {:.4} {:.4} {:.4} c", x, y + r - r * K, x + r - r * K, y, x + r, y).unwrap();
    writeln!(data, "h").unwrap();
}

fn draw_rounded_rect_ccw(data: &mut String, x: f32, y: f32, w: f32, h: f32, r: f32) {
    // Counter-Clockwise
    let x2 = x + w;
    let y2 = y + h;
    
    // Start Top-Right (ish), go Left
    writeln!(data, "{:.4} {:.4} m", x2 - r, y).unwrap();
    writeln!(data, "{:.4} {:.4} l", x + r, y).unwrap();
    // Curve to Left-Top
    writeln!(data, "{:.4} {:.4} {:.4} {:.4} {:.4} {:.4} c", x + r - r * K, y, x, y + r - r * K, x, y + r).unwrap();
    // Down
    writeln!(data, "{:.4} {:.4} l", x, y2 - r).unwrap();
    // Curve to Bottom-Left
    writeln!(data, "{:.4} {:.4} {:.4} {:.4} {:.4} {:.4} c", x, y2 - r + r * K, x + r - r * K, y2, x + r, y2).unwrap();
    // Right
    writeln!(data, "{:.4} {:.4} l", x2 - r, y2).unwrap();
    // Curve to Right-Bottom
    writeln!(data, "{:.4} {:.4} {:.4} {:.4} {:.4} {:.4} c", x2 - r + r * K, y2, x2, y2 - r + r * K, x2, y2 - r).unwrap();
    // Up
    writeln!(data, "{:.4} {:.4} l", x2, y + r).unwrap();
    // Curve to Top-Right
    writeln!(data, "{:.4} {:.4} {:.4} {:.4} {:.4} {:.4} c", x2, y + r - r * K, x2 - r + r * K, y, x2 - r, y).unwrap();
    writeln!(data, "h").unwrap();
}
