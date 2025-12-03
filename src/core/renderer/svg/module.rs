use svg::node::element::path::Data;
use crate::models::ModuleShape;
use crate::core::renderer::ModuleContext;

pub fn append_module_path(data: &mut Data, shape: ModuleShape, x: f32, y: f32, size: f32, ctx: &ModuleContext) {
    match shape {
        ModuleShape::Square => {
            *data = data.clone().move_to((x, y))
                .line_to((x + size, y))
                .line_to((x + size, y + size))
                .line_to((x, y + size))
                .close();
        }
        ModuleShape::Dots => {
            let r = size / 2.0 - (size * 0.05);
            let cx = x + size / 2.0;
            let cy = y + size / 2.0;
            *data = data.clone().move_to((cx - r, cy))
                .elliptical_arc_to((r, r, 0, 1, 0, cx + r, cy))
                .elliptical_arc_to((r, r, 0, 1, 0, cx - r, cy));
        }
        ModuleShape::Gapped => {
            let s = size - (size * 0.1);
            let r = size * 0.1;
            *data = data.clone().move_to((x + r, y))
                .line_to((x + s - r, y))
                .quadratic_curve_to((x + s, y, x + s, y + r))
                .line_to((x + s, y + s - r))
                .quadratic_curve_to((x + s, y + s, x + s - r, y + s))
                .line_to((x + r, y + s))
                .quadratic_curve_to((x, y + s, x, y + s - r))
                .line_to((x, y + r))
                .quadratic_curve_to((x, y, x + r, y))
                .close();
        }
        ModuleShape::Diamond => {
            *data = data.clone().move_to((x + size / 2.0, y))
                .line_to((x + size, y + size / 2.0))
                .line_to((x + size / 2.0, y + size))
                .line_to((x, y + size / 2.0))
                .close();
        }
        ModuleShape::HorizontalBars => {
            let height = size * 0.6;
            let y_offset = (size - height) / 2.0;
            let r = height / 2.0;
            let y_top = y + y_offset;
            let y_bottom = y + y_offset + height;
            let x_end = x + size;
            if ctx.left {
                *data = data.clone().move_to((x, y_top));
            } else {
                *data = data.clone().move_to((x + r, y_top));
            }
            if ctx.right {
                *data = data.clone().line_to((x_end, y_top));
            } else {
                *data = data.clone().line_to((x_end - r, y_top));
            }
            if !ctx.right {
                *data = data.clone().quadratic_curve_to((x_end, y_top, x_end, y_top + r))
                    .quadratic_curve_to((x_end, y_bottom, x_end - r, y_bottom));
            } else {
                *data = data.clone().line_to((x_end, y_bottom));
            }
            if ctx.left {
                *data = data.clone().line_to((x, y_bottom));
            } else {
                *data = data.clone().line_to((x + r, y_bottom));
            }
            if !ctx.left {
                *data = data.clone().quadratic_curve_to((x, y_bottom, x, y_bottom - r))
                    .quadratic_curve_to((x, y_top, x + r, y_top));
            } else {
                *data = data.clone().line_to((x, y_top));
            }
            *data = data.clone().close();
        }
        ModuleShape::VerticalBars => {
            let width = size * 0.6;
            let x_offset = (size - width) / 2.0;
            let r = width / 2.0;
            let x_left = x + x_offset;
            let x_right = x + x_offset + width;
            let y_end = y + size;
            if ctx.top {
                *data = data.clone().move_to((x_left, y));
            } else {
                *data = data.clone().move_to((x_left, y + r));
            }
            if !ctx.top {
                *data = data.clone().quadratic_curve_to((x_left, y, x_left + r, y))
                    .quadratic_curve_to((x_right, y, x_right, y + r));
            } else {
                *data = data.clone().line_to((x_right, y));
            }
            if ctx.bottom {
                *data = data.clone().line_to((x_right, y_end));
            } else {
                *data = data.clone().line_to((x_right, y_end - r));
            }
            if !ctx.bottom {
                *data = data.clone().quadratic_curve_to((x_right, y_end, x_right - r, y_end))
                    .quadratic_curve_to((x_left, y_end, x_left, y_end - r));
            } else {
                *data = data.clone().line_to((x_left, y_end));
            }
            if ctx.top {
                *data = data.clone().line_to((x_left, y));
            } else {
                *data = data.clone().line_to((x_left, y + r));
            }
            *data = data.clone().close();
        }
        ModuleShape::Heart => {
            let s = size;
            let s_half = s / 2.0;
            *data = data.clone().move_to((x + s_half, y + s * 0.3))
                .cubic_curve_to((x + s_half, y, x + s * 0.95, y, x + s * 0.95, y + s * 0.3))
                .cubic_curve_to((
                    x + s * 0.95,
                    y + s * 0.6,
                    x + s * 0.65,
                    y + s * 0.9,
                    x + s_half,
                    y + s,
                ))
                .cubic_curve_to((
                    x + s * 0.35,
                    y + s * 0.9,
                    x + s * 0.05,
                    y + s * 0.6,
                    x + s * 0.05,
                    y + s * 0.3,
                ))
                .cubic_curve_to((x + s * 0.05, y, x + s_half, y, x + s_half, y + s * 0.3))
                .close();
        }
    }
}
