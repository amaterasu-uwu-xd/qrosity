use crate::models::ModuleShape;
use crate::core::renderer::ModuleContext;
use std::fmt::Write;

pub fn append_module_path(data: &mut String, shape: ModuleShape, x: f32, y: f32, size: f32, ctx: &ModuleContext) {
    match shape {
        ModuleShape::Square => {
            let _ = write!(data, "M{x} {y} L{x1} {y} L{x1} {y1} L{x} {y1} Z ", x=x, y=y, x1=x+size, y1=y+size);
        }
        ModuleShape::Dots => {
            let r = size / 2.0 - (size * 0.05);
            let cx = x + size / 2.0;
            let cy = y + size / 2.0;
            // A rx ry x-axis-rotation large-arc-flag sweep-flag x y
            let _ = write!(data, "M{mx} {my} A{r} {r} 0 1 0 {ax1} {ay1} A{r} {r} 0 1 0 {mx} {my} ", 
                mx=cx-r, my=cy, r=r, ax1=cx+r, ay1=cy);
        }
        ModuleShape::Gapped => {
            let s = size - (size * 0.1);
            let r = size * 0.1;
            let _ = write!(data, "M{mx} {my} L{lx1} {ly1} Q{qx1} {qy1} {qx2} {qy2} L{lx2} {ly2} Q{qx3} {qy3} {qx4} {qy4} L{lx3} {ly3} Q{qx5} {qy5} {qx6} {qy6} L{lx4} {ly4} Q{qx7} {qy7} {qx8} {qy8} Z ",
                mx=x+r, my=y,
                lx1=x+s-r, ly1=y,
                qx1=x+s, qy1=y, qx2=x+s, qy2=y+r,
                lx2=x+s, ly2=y+s-r,
                qx3=x+s, qy3=y+s, qx4=x+s-r, qy4=y+s,
                lx3=x+r, ly3=y+s,
                qx5=x, qy5=y+s, qx6=x, qy6=y+s-r,
                lx4=x, ly4=y+r,
                qx7=x, qy7=y, qx8=x+r, qy8=y
            );
        }
        ModuleShape::Diamond => {
            let _ = write!(data, "M{mx} {my} L{lx1} {ly1} L{lx2} {ly2} L{lx3} {ly3} Z ",
                mx=x+size/2.0, my=y,
                lx1=x+size, ly1=y+size/2.0,
                lx2=x+size/2.0, ly2=y+size,
                lx3=x, ly3=y+size/2.0
            );
        }
        ModuleShape::HorizontalBars => {
            let height = size * 0.6;
            let y_offset = (size - height) / 2.0;
            let r = height / 2.0;
            let y_top = y + y_offset;
            let y_bottom = y + y_offset + height;
            let x_end = x + size;
            
            if ctx.left {
                let _ = write!(data, "M{} {} ", x, y_top);
            } else {
                let _ = write!(data, "M{} {} ", x + r, y_top);
            }
            
            if ctx.right {
                let _ = write!(data, "L{} {} ", x_end, y_top);
            } else {
                let _ = write!(data, "L{} {} ", x_end - r, y_top);
            }
            
            if !ctx.right {
                let _ = write!(data, "Q{} {} {} {} Q{} {} {} {} ", x_end, y_top, x_end, y_top + r, x_end, y_bottom, x_end - r, y_bottom);
            } else {
                let _ = write!(data, "L{} {} ", x_end, y_bottom);
            }
            
            if ctx.left {
                let _ = write!(data, "L{} {} ", x, y_bottom);
            } else {
                let _ = write!(data, "L{} {} ", x + r, y_bottom);
            }
            
            if !ctx.left {
                let _ = write!(data, "Q{} {} {} {} Q{} {} {} {} ", x, y_bottom, x, y_bottom - r, x, y_top, x + r, y_top);
            } else {
                let _ = write!(data, "L{} {} ", x, y_top);
            }
            let _ = write!(data, "Z ");
        }
        ModuleShape::VerticalBars => {
            let width = size * 0.6;
            let x_offset = (size - width) / 2.0;
            let r = width / 2.0;
            let x_left = x + x_offset;
            let x_right = x + x_offset + width;
            let y_end = y + size;
            
            if ctx.top {
                let _ = write!(data, "M{} {} ", x_left, y);
            } else {
                let _ = write!(data, "M{} {} ", x_left, y + r);
            }
            
            if !ctx.top {
                let _ = write!(data, "Q{} {} {} {} Q{} {} {} {} ", x_left, y, x_left + r, y, x_right, y, x_right, y + r);
            } else {
                let _ = write!(data, "L{} {} ", x_right, y);
            }
            
            if ctx.bottom {
                let _ = write!(data, "L{} {} ", x_right, y_end);
            } else {
                let _ = write!(data, "L{} {} ", x_right, y_end - r);
            }
            
            if !ctx.bottom {
                let _ = write!(data, "Q{} {} {} {} Q{} {} {} {} ", x_right, y_end, x_right - r, y_end, x_left, y_end, x_left, y_end - r);
            } else {
                let _ = write!(data, "L{} {} ", x_left, y_end);
            }
            
            if ctx.top {
                let _ = write!(data, "L{} {} ", x_left, y);
            } else {
                let _ = write!(data, "L{} {} ", x_left, y + r);
            }
            let _ = write!(data, "Z ");
        }
        ModuleShape::Heart => {
            let s = size;
            let s_half = s / 2.0;
            let _ = write!(data, "M{} {} C{} {} {} {} {} {} C{} {} {} {} {} {} C{} {} {} {} {} {} C{} {} {} {} {} {} Z ",
                x + s_half, y + s * 0.3,
                x + s_half, y, x + s * 0.95, y, x + s * 0.95, y + s * 0.3,
                x + s * 0.95, y + s * 0.6, x + s * 0.65, y + s * 0.9, x + s_half, y + s,
                x + s * 0.35, y + s * 0.9, x + s * 0.05, y + s * 0.6, x + s * 0.05, y + s * 0.3,
                x + s * 0.05, y, x + s_half, y, x + s_half, y + s * 0.3
            );
        }
    }
}
