use crate::models::FinderShape;
use std::fmt::Write;

pub fn append_finder_path(data: &mut String, shape: FinderShape, x: f32, y: f32, size: f32) {
    let module_count = 7.0;
    let total_size = size * module_count;
    
    match shape {
        FinderShape::Square => {
            writeln!(data, "{} {} moveto {} {} lineto {} {} lineto {} {} lineto closepath", 
                x, y, x + total_size, y, x + total_size, y + total_size, x, y + total_size).unwrap();
            
            let inner_start = size;
            let inner_end = size * 6.0;
            writeln!(data, "{} {} moveto {} {} lineto {} {} lineto {} {} lineto closepath",
                x + inner_start, y + inner_start, 
                x + inner_start, y + inner_end,
                x + inner_end, y + inner_end,
                x + inner_end, y + inner_start).unwrap();                
            let center_start = size * 2.0;
            let center_end = size * 5.0;
            writeln!(data, "{} {} moveto {} {} lineto {} {} lineto {} {} lineto closepath",
                x + center_start, y + center_start,
                x + center_end, y + center_start,
                x + center_end, y + center_end,
                x + center_start, y + center_end).unwrap();
        }
        FinderShape::Circle => {
             let cx = x + total_size / 2.0;
             let cy = y + total_size / 2.0;
             
             // Outer circle
             writeln!(data, "{} {} {} 0 360 arc closepath", cx, cy, total_size / 2.0).unwrap();
             // Inner hole (reverse)
             writeln!(data, "{} {} {} 360 0 arcn closepath", cx, cy, size * 2.5).unwrap();
             
             // Center circle
             writeln!(data, "{} {} {} 0 360 arc closepath", cx, cy, size * 1.5).unwrap();
        }
        FinderShape::Rounded => {
            let r_outer = size; // 1.0 module
            
            // Outer rounded rect (CW)
            writeln!(data, "{} {} moveto", x + r_outer, y).unwrap();
            writeln!(data, "{} {} {} {} {} arct", x + total_size, y, x + total_size, y + total_size, r_outer).unwrap();
            writeln!(data, "{} {} {} {} {} arct", x + total_size, y + total_size, x, y + total_size, r_outer).unwrap();
            writeln!(data, "{} {} {} {} {} arct", x, y + total_size, x, y, r_outer).unwrap();
            writeln!(data, "{} {} {} {} {} arct", x, y, x + total_size, y, r_outer).unwrap();
            writeln!(data, "closepath").unwrap();
            
            // Inner hole (CCW - Reverse)
            let inner_start = size;
            let inner_size = size * 5.0;
            let r_inner = size * 0.7;
            let xi = x + inner_start;
            let yi = y + inner_start;
            writeln!(data, "{} {} moveto", xi + r_inner, yi).unwrap();
            // Top-Left corner
            writeln!(data, "{} {} {} {} {} arct", xi, yi, xi, yi + inner_size, r_inner).unwrap();
            // Bottom-Left corner
            writeln!(data, "{} {} {} {} {} arct", xi, yi + inner_size, xi + inner_size, yi + inner_size, r_inner).unwrap();
            // Bottom-Right corner
            writeln!(data, "{} {} {} {} {} arct", xi + inner_size, yi + inner_size, xi + inner_size, yi, r_inner).unwrap();
            // Top-Right corner
            writeln!(data, "{} {} {} {} {} arct", xi + inner_size, yi, xi, yi, r_inner).unwrap();
            writeln!(data, "closepath").unwrap();
                
            // Center rounded rect (CW)
            let center_start = size * 2.0;
            let center_size = size * 3.0;
            let r_center = size * 0.5;
            let xc = x + center_start;
            let yc = y + center_start;
            
            writeln!(data, "{} {} moveto", xc + r_center, yc).unwrap();
            writeln!(data, "{} {} {} {} {} arct", xc + center_size, yc, xc + center_size, yc + center_size, r_center).unwrap();
            writeln!(data, "{} {} {} {} {} arct", xc + center_size, yc + center_size, xc, yc + center_size, r_center).unwrap();
            writeln!(data, "{} {} {} {} {} arct", xc, yc + center_size, xc, yc, r_center).unwrap();
            writeln!(data, "{} {} {} {} {} arct", xc, yc, xc + center_size, yc, r_center).unwrap();
            writeln!(data, "closepath").unwrap();
        }
    }
}
