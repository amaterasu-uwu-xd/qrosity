use tiny_skia::{Path, PathBuilder, Rect};
use crate::models::ModuleShape;

pub struct ModuleContext {
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
}

pub fn draw_module(shape: ModuleShape, x: f32, y: f32, size: f32, ctx: &ModuleContext) -> Path {
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
        ModuleShape::HorizontalBars => {
            let height = size * 0.6;
            let y_offset = (size - height) / 2.0;
            let radius = height / 2.0;

            let x_start = x;
            let x_end = x + size;
            let y_top = y + y_offset;
            let y_bottom = y + y_offset + height;

            // Start point (top-left)
            if ctx.left {
                pb.move_to(x_start, y_top);
            } else {
                pb.move_to(x_start + radius, y_top);
            }

            // Top edge
            pb.line_to(if ctx.right { x_end } else { x_end - radius }, y_top);

            // Right side
            if !ctx.right {
                pb.quad_to(x_end, y_top, x_end, y_top + radius);
                pb.quad_to(x_end, y_bottom, x_end - radius, y_bottom);
            } else {
                pb.line_to(x_end, y_bottom);
            }

            // Bottom edge
            pb.line_to(if ctx.left { x_start } else { x_start + radius }, y_bottom);

            // Left side
            if !ctx.left {
                pb.quad_to(x_start, y_bottom, x_start, y_bottom - radius);
                pb.quad_to(x_start, y_top, x_start + radius, y_top);
            } else {
                pb.line_to(x_start, y_top);
            }
            
            pb.close();
        }
        ModuleShape::VerticalBars => {
            let width = size * 0.6;
            let x_offset = (size - width) / 2.0;
            let radius = width / 2.0;

            let x_left = x + x_offset;
            let x_right = x + x_offset + width;
            let y_start = y;
            let y_end = y + size;

            // Start point (top-left)
            if ctx.top {
                pb.move_to(x_left, y_start);
            } else {
                pb.move_to(x_left, y_start + radius);
            }

            // Top side
            if !ctx.top {
                pb.quad_to(x_left, y_start, x_left + radius, y_start);
                pb.quad_to(x_right, y_start, x_right, y_start + radius);
            } else {
                pb.line_to(x_right, y_start);
            }

            // Right edge
            pb.line_to(x_right, if ctx.bottom { y_end } else { y_end - radius });

            // Bottom side
            if !ctx.bottom {
                pb.quad_to(x_right, y_end, x_right - radius, y_end);
                pb.quad_to(x_left, y_end, x_left, y_end - radius);
            } else {
                pb.line_to(x_right, y_end);
                pb.line_to(x_left, y_end);
            }

            // Left edge
            pb.line_to(x_left, if ctx.top { y_start } else { y_start + radius });
            
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
