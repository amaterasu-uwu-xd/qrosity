use tiny_skia::{Path, PathBuilder, Rect};
use crate::models::ModuleShape;

pub fn draw_module(shape: ModuleShape, x: f32, y: f32, size: f32) -> Path {
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
