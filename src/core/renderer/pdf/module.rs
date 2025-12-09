use crate::models::ModuleShape;
use crate::core::renderer::ModuleContext;
use std::fmt::Write;

// Constant for Bezier curve approximation of a circle (4/3 * tan(pi/8))
const K: f32 = 0.5522847498;

pub fn append_module_path(data: &mut String, shape: ModuleShape, x: f32, y: f32, size: f32, ctx: &ModuleContext) {
    match shape {
        ModuleShape::Square => {
            // Add a small overlap to avoid stitching artifacts (white lines between modules)
            let overlap = size * 0.02;
            writeln!(data, "{:.4} {:.4} {:.4} {:.4} re", x - overlap, y - overlap, size + 2.0 * overlap, size + 2.0 * overlap).unwrap();
        }
        ModuleShape::Dots => {
            let r = size / 2.0 - (size * 0.05);
            let cx = x + size / 2.0;
            let cy = y + size / 2.0;
            draw_circle(data, cx, cy, r);
        }
        ModuleShape::Gapped => {
            let s = size - (size * 0.1);
            let r = size * 0.1;
            let offset = (size - s) / 2.0;
            let x = x + offset;
            let y = y + offset;
            draw_rounded_rect(data, x, y, s, s, r);
        }
        ModuleShape::Diamond => {
            let cx = x + size / 2.0;
            let cy = y + size / 2.0;
            writeln!(data, "{:.4} {:.4} m", cx, y).unwrap();
            writeln!(data, "{:.4} {:.4} l", x + size, cy).unwrap();
            writeln!(data, "{:.4} {:.4} l", cx, y + size).unwrap();
            writeln!(data, "{:.4} {:.4} l", x, cy).unwrap();
            writeln!(data, "h").unwrap();
        }
        ModuleShape::HorizontalBars => {
            let height = size * 0.6;
            let y_offset = (size - height) / 2.0;
            let r = height / 2.0;
            let y_top = y + y_offset;
            let y_bottom = y + y_offset + height;
            
            // Overlap for connected bars
            let overlap = size * 0.05;
            
            let x_start = if ctx.left { x - overlap } else { x + r };
            let x_finish = if ctx.right { x + size + overlap } else { x + size - r };
            
            writeln!(data, "{:.4} {:.4} m", x_start, y_top).unwrap();
            writeln!(data, "{:.4} {:.4} l", x_finish, y_top).unwrap();
            
            if !ctx.right {
                // Draw right semi-circle cap
                let cx = x_finish;
                let cy = y + size / 2.0;
                writeln!(data, "{:.4} {:.4} {:.4} {:.4} {:.4} {:.4} c", 
                    cx + r * K, y_top, 
                    cx + r, cy - r * K, 
                    cx + r, cy).unwrap();
                writeln!(data, "{:.4} {:.4} {:.4} {:.4} {:.4} {:.4} c", 
                    cx + r, cy + r * K, 
                    cx + r * K, y_bottom, 
                    cx, y_bottom).unwrap();
            } else {
                writeln!(data, "{:.4} {:.4} l", x_finish, y_bottom).unwrap();
            }
            
            writeln!(data, "{:.4} {:.4} l", x_start, y_bottom).unwrap();
            
            if !ctx.left {
                // Draw left semi-circle cap
                let cx = x_start;
                let cy = y + size / 2.0;
                writeln!(data, "{:.4} {:.4} {:.4} {:.4} {:.4} {:.4} c", 
                    cx - r * K, y_bottom, 
                    cx - r, cy + r * K, 
                    cx - r, cy).unwrap();
                writeln!(data, "{:.4} {:.4} {:.4} {:.4} {:.4} {:.4} c", 
                    cx - r, cy - r * K, 
                    cx - r * K, y_top, 
                    cx, y_top).unwrap();
            } else {
                writeln!(data, "{:.4} {:.4} l", x_start, y_top).unwrap();
            }
            writeln!(data, "h").unwrap();
        }
        ModuleShape::VerticalBars => {
            let width = size * 0.6;
            let x_offset = (size - width) / 2.0;
            let r = width / 2.0;
            let x_left = x + x_offset;
            let x_right = x + x_offset + width;
            
            // Overlap for connected bars
            let overlap = size * 0.05;

            let y_start = if ctx.top { y - overlap } else { y + r };
            let y_finish = if ctx.bottom { y + size + overlap } else { y + size - r };
            
            writeln!(data, "{:.4} {:.4} m", x_right, y_start).unwrap();
            
            if !ctx.top {
                // Top cap
                let cx = x + size / 2.0;
                let cy = y_start;
                writeln!(data, "{:.4} {:.4} {:.4} {:.4} {:.4} {:.4} c", 
                    x_right, cy - r * K, 
                    cx + r * K, cy - r, 
                    cx, cy - r).unwrap();
                writeln!(data, "{:.4} {:.4} {:.4} {:.4} {:.4} {:.4} c", 
                    cx - r * K, cy - r, 
                    x_left, cy - r * K, 
                    x_left, cy).unwrap();
            } else {
                writeln!(data, "{:.4} {:.4} l", x_left, y_start).unwrap();
            }
            
            writeln!(data, "{:.4} {:.4} l", x_left, y_finish).unwrap();
            
            if !ctx.bottom {
                // Bottom cap
                let cx = x + size / 2.0;
                let cy = y_finish;
                writeln!(data, "{:.4} {:.4} {:.4} {:.4} {:.4} {:.4} c", 
                    x_left, cy + r * K, 
                    cx - r * K, cy + r, 
                    cx, cy + r).unwrap();
                writeln!(data, "{:.4} {:.4} {:.4} {:.4} {:.4} {:.4} c", 
                    cx + r * K, cy + r, 
                    x_right, cy + r * K, 
                    x_right, cy).unwrap();
            } else {
                writeln!(data, "{:.4} {:.4} l", x_right, y_finish).unwrap();
            }
            
            writeln!(data, "h").unwrap();
        }
        ModuleShape::Heart => {
            let s = size;
            let s_half = s / 2.0;
            
            // M x+s_half, y+s*0.3
            writeln!(data, "{:.4} {:.4} m", x + s_half, y + s * 0.3).unwrap();
            
            // C x+s_half, y, x+s*0.95, y, x+s*0.95, y+s*0.3
            writeln!(data, "{:.4} {:.4} {:.4} {:.4} {:.4} {:.4} c", 
                x + s_half, y, 
                x + s * 0.95, y, 
                x + s * 0.95, y + s * 0.3).unwrap();
                
            // C x+s*0.95, y+s*0.6, x+s*0.65, y+s*0.9, x+s_half, y+s
            writeln!(data, "{:.4} {:.4} {:.4} {:.4} {:.4} {:.4} c", 
                x + s * 0.95, y + s * 0.6, 
                x + s * 0.65, y + s * 0.9, 
                x + s_half, y + s).unwrap();
                
            // C x+s*0.35, y+s*0.9, x+s*0.05, y+s*0.6, x+s*0.05, y+s*0.3
            writeln!(data, "{:.4} {:.4} {:.4} {:.4} {:.4} {:.4} c", 
                x + s * 0.35, y + s * 0.9, 
                x + s * 0.05, y + s * 0.6, 
                x + s * 0.05, y + s * 0.3).unwrap();
                
            // C x+s*0.05, y, x+s_half, y, x+s_half, y+s*0.3
            writeln!(data, "{:.4} {:.4} {:.4} {:.4} {:.4} {:.4} c", 
                x + s * 0.05, y, 
                x + s_half, y, 
                x + s_half, y + s * 0.3).unwrap();
                
            writeln!(data, "h").unwrap();
        }
    }
}

fn draw_circle(data: &mut String, cx: f32, cy: f32, r: f32) {
    writeln!(data, "{:.4} {:.4} m", cx + r, cy).unwrap();
    writeln!(data, "{:.4} {:.4} {:.4} {:.4} {:.4} {:.4} c", cx + r, cy + r * K, cx + r * K, cy + r, cx, cy + r).unwrap();
    writeln!(data, "{:.4} {:.4} {:.4} {:.4} {:.4} {:.4} c", cx - r * K, cy + r, cx - r, cy + r * K, cx - r, cy).unwrap();
    writeln!(data, "{:.4} {:.4} {:.4} {:.4} {:.4} {:.4} c", cx - r, cy - r * K, cx - r * K, cy - r, cx, cy - r).unwrap();
    writeln!(data, "{:.4} {:.4} {:.4} {:.4} {:.4} {:.4} c", cx + r * K, cy - r, cx + r, cy - r * K, cx + r, cy).unwrap();
    writeln!(data, "h").unwrap();
}

fn draw_rounded_rect(data: &mut String, x: f32, y: f32, w: f32, h: f32, r: f32) {
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
