use crate::models::GradientDirection;
use image::DynamicImage;

pub fn parse_hex_color(hex: &str) -> Option<(u8, u8, u8)> {
    let hex = hex.trim_start_matches('#');
    if hex.len() == 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some((r, g, b))
    } else if hex.len() == 3 {
        let r = u8::from_str_radix(&hex[0..1], 16).ok()?;
        let g = u8::from_str_radix(&hex[1..2], 16).ok()?;
        let b = u8::from_str_radix(&hex[2..3], 16).ok()?;
        Some((r * 17, g * 17, b * 17))
    } else {
        None
    }
}

pub fn load_image(path: &str) -> Result<DynamicImage, String> {
    image::open(path).map_err(|e| e.to_string())
}

pub fn load_raster_icon(path: &str, output_format_name: &str) -> Option<DynamicImage> {
    if !std::path::Path::new(path).exists() {
        eprintln!("Warning: Icon file not found: {}", path);
        return None;
    }

    if is_svg_file(path) {
        eprintln!(
            "Warning: SVG icons are not supported in {} output. The icon will be ignored.",
            output_format_name
        );
        return None;
    }

    match load_image(path) {
        Ok(img) => Some(img),
        Err(e) => {
            eprintln!(
                "Warning: Failed to load icon '{}': {}. The icon will be ignored.",
                path, e
            );
            None
        }
    }
}

pub fn read_icon_data(path: &str) -> Option<Vec<u8>> {
    if !std::path::Path::new(path).exists() {
        eprintln!("Warning: Icon file not found: {}", path);
        return None;
    }

    match std::fs::read(path) {
        Ok(data) => Some(data),
        Err(e) => {
            eprintln!(
                "Warning: Failed to read icon '{}': {}. The icon will be ignored.",
                path, e
            );
            None
        }
    }
}

pub fn is_svg_buffer(buffer: &[u8]) -> bool {
    if let Ok(s) = std::str::from_utf8(buffer) {
        let s = s.trim_start();
        s.starts_with('<') && s.contains("<svg")
    } else {
        false
    }
}

pub fn is_svg_file(path: &str) -> bool {
    use std::io::Read;
    if let Ok(file) = std::fs::File::open(path) {
        let mut buffer = Vec::new();
        if file.take(4096).read_to_end(&mut buffer).is_ok() {
            return is_svg_buffer(&buffer);
        }
    }
    false
}

pub fn get_gradient_coords(
    direction: GradientDirection,
    width: f32,
    height: f32,
) -> (f32, f32, f32, f32, f32, f32) {
    // Returns (x0, y0, r0, x1, y1, r1)
    match direction {
        GradientDirection::TopToBottom => (0.0, 0.0, 0.0, 0.0, height, 0.0),
        GradientDirection::LeftToRight => (0.0, 0.0, 0.0, width, 0.0, 0.0),
        GradientDirection::TopLeftToBottomRight => (0.0, 0.0, 0.0, width, height, 0.0),
        GradientDirection::BottomLeftToTopRight => (0.0, height, 0.0, width, 0.0, 0.0),
        GradientDirection::Radial => (
            width / 2.0,
            height / 2.0,
            0.0,
            width / 2.0,
            height / 2.0,
            width / 2.0 * 1.414,
        ),
    }
}

pub fn generate_pdf_ps_gradient_function(colors: &[(f32, f32, f32)]) -> String {
    let n = colors.len();
    if n < 2 {
        return String::new();
    }

    let mut s = String::new();

    // Stitching Function (Type 3)
    s.push_str("<< /FunctionType 3 /Domain [0 1] ");

    // Functions array
    s.push_str("/Functions [");
    for i in 0..n - 1 {
        let c0 = colors[i];
        let c1 = colors[i + 1];
        s.push_str(&format!(" << /FunctionType 2 /Domain [0 1] /C0 [{:.3} {:.3} {:.3}] /C1 [{:.3} {:.3} {:.3}] /N 1 >>",
            c0.0, c0.1, c0.2, c1.0, c1.1, c1.2));
    }
    s.push_str(" ] ");

    // Bounds array
    s.push_str("/Bounds [");
    for i in 1..n - 1 {
        let val = i as f32 / (n - 1) as f32;
        s.push_str(&format!(" {:.3}", val));
    }
    s.push_str(" ] ");

    // Encode array
    s.push_str("/Encode [");
    for _ in 0..n - 1 {
        s.push_str(" 0 1");
    }
    s.push_str(" ] >>");

    s
}
