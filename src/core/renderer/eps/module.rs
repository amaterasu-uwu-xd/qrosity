use crate::models::ModuleShape;
use crate::core::renderer::ModuleContext;
use std::fmt::Write;

pub fn append_module_path(data: &mut String, shape: ModuleShape, x: f32, y: f32, size: f32, ctx: &ModuleContext) {
    match shape {
        ModuleShape::Square => {
            let x1 = x + size;
            let y1 = y + size;
            writeln!(data, "{} {} moveto {} {} lineto {} {} lineto {} {} lineto closepath", x, y, x1, y, x1, y1, x, y1).unwrap();
        }
        ModuleShape::Dots => {
            let r = size / 2.0 - (size * 0.05);
            let cx = x + size / 2.0;
            let cy = y + size / 2.0;
            writeln!(data, "{} {} {} 0 360 arc closepath", cx, cy, r).unwrap();
        }
        ModuleShape::Gapped => {
            // Rounded square with gaps
            let s = size - (size * 0.1);
            let r = size * 0.1;
            let offset = (size - s) / 2.0;
            let x = x + offset;
            let y = y + offset;
            
            // Using arct for rounded corners
            // Start at top edge
            writeln!(data, "{} {} moveto", x + r, y).unwrap();
            writeln!(data, "{} {} {} {} {} arct", x + s, y, x + s, y + s, r).unwrap();
            writeln!(data, "{} {} {} {} {} arct", x + s, y + s, x, y + s, r).unwrap();
            writeln!(data, "{} {} {} {} {} arct", x, y + s, x, y, r).unwrap();
            writeln!(data, "{} {} {} {} {} arct", x, y, x + s, y, r).unwrap();
            writeln!(data, "closepath").unwrap();
        }
        ModuleShape::Diamond => {
            let cx = x + size / 2.0;
            let cy = y + size / 2.0;
            writeln!(data, "{} {} moveto {} {} lineto {} {} lineto {} {} lineto closepath", 
                cx, y, x + size, cy, cx, y + size, x, cy).unwrap();
        }
        ModuleShape::HorizontalBars => {
            let height = size * 0.6;
            let y_offset = (size - height) / 2.0;
            let r = height / 2.0;
            let y_top = y + y_offset;
            let y_bottom = y + y_offset + height;
            let x_end = x + size;
            if ctx.left {
                writeln!(data, "{} {} moveto", x, y_top).unwrap();
            } else {
                writeln!(data, "{} {} moveto", x + r, y_top).unwrap();
            }
            if ctx.right {
                writeln!(data, "{} {} lineto", x_end, y_top).unwrap();
            } else {
                writeln!(data, "{} {} lineto", x_end - r, y_top).unwrap();
            }
            if ctx.right {
                writeln!(data, "{} {} lineto", x_end, y_bottom).unwrap();
            } else {
                writeln!(data, "{} {} {} {} {} arct", x_end, y_top, x_end, y_bottom, r).unwrap();
                writeln!(data, "{} {} {} {} {} arct", x_end, y_bottom, x_end - r, y_bottom, r).unwrap();
            }
            if ctx.left {
                writeln!(data, "{} {} lineto", x, y_bottom).unwrap();
            } else {
                writeln!(data, "{} {} lineto", x + r, y_bottom).unwrap();
            }
            if ctx.left {
                writeln!(data, "{} {} lineto", x, y_top).unwrap();
            } else {
                writeln!(data, "{} {} {} {} {} arct", x, y_bottom, x, y_top, r).unwrap();
                writeln!(data, "{} {} {} {} {} arct", x, y_top, x + r, y_top, r).unwrap();
            }
            
            writeln!(data, "closepath").unwrap();
        }
        ModuleShape::VerticalBars => {
            let width = size * 0.6;
            let x_offset = (size - width) / 2.0;
            let r = width / 2.0;
            let x_left = x + x_offset;
            let x_right = x + x_offset + width;
            let y_end = y + size;
            if ctx.top {
                writeln!(data, "{} {} moveto", x_left, y).unwrap();
            } else {
                writeln!(data, "{} {} moveto", x_left, y + r).unwrap();
            }
            if ctx.top {
                writeln!(data, "{} {} lineto", x_right, y).unwrap();
            } else {
                writeln!(data, "{} {} {} {} {} arct", x_left, y, x_right, y, r).unwrap();
                writeln!(data, "{} {} {} {} {} arct", x_right, y, x_right, y + r, r).unwrap();
            }
            if ctx.bottom {
                writeln!(data, "{} {} lineto", x_right, y_end).unwrap();
            } else {
                writeln!(data, "{} {} lineto", x_right, y_end - r).unwrap();
            }
            if ctx.bottom {
                writeln!(data, "{} {} lineto", x_left, y_end).unwrap();
            } else {
                writeln!(data, "{} {} {} {} {} arct", x_right, y_end, x_left, y_end, r).unwrap();
                writeln!(data, "{} {} {} {} {} arct", x_left, y_end, x_left, y_end - r, r).unwrap();
            }
            if ctx.top {
                writeln!(data, "{} {} lineto", x_left, y).unwrap();
            } else {
                writeln!(data, "{} {} lineto", x_left, y + r).unwrap();
            }
            writeln!(data, "closepath").unwrap();
        }
        ModuleShape::Heart => {
            let s = size;
            let s_half = s / 2.0;
            writeln!(data, "{} {} moveto", x + s_half, y + s * 0.3).unwrap();
            writeln!(data, "{} {} {} {} {} {} curveto", 
                x + s_half, y, 
                x + s * 0.95, y, 
                x + s * 0.95, y + s * 0.3
            ).unwrap();
            writeln!(data, "{} {} {} {} {} {} curveto", 
                x + s * 0.95, y + s * 0.6, 
                x + s * 0.65, y + s * 0.9, 
                x + s_half, y + s
            ).unwrap();
            writeln!(data, "{} {} {} {} {} {} curveto", 
                x + s * 0.35, y + s * 0.9, 
                x + s * 0.05, y + s * 0.6, 
                x + s * 0.05, y + s * 0.3
            ).unwrap();
            writeln!(data, "{} {} {} {} {} {} curveto", 
                x + s * 0.05, y, 
                x + s_half, y, 
                x + s_half, y + s * 0.3
            ).unwrap();
            writeln!(data, "closepath").unwrap();
        }
    }
}
