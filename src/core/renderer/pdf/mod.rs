use crate::core::renderer::{QrGrid, QrRenderer, utils};
use crate::models::{GradientDirection, QrConfig};
use image::{ColorType, DynamicImage, GenericImageView};
use std::fs::File;
use std::io::Write;

mod finder;
mod module;

use finder::append_finder_path;
use module::append_module_path;

pub struct PdfRenderer {
    data: Vec<u8>,
}

impl PdfRenderer {
    pub fn new<G: QrGrid + ?Sized>(grid: &G, options: &QrConfig) -> Result<Self, String> {
        let pdf_content = render_pdf(grid, options, options.ppm as f32)?;
        Ok(Self { data: pdf_content })
    }
}

impl QrRenderer for PdfRenderer {
    fn save(&self, path: &str) -> Result<String, String> {
        let mut final_path = path.to_string();
        if !final_path.to_lowercase().ends_with(".pdf") {
            final_path.push_str(".pdf");
        }

        let mut file = File::create(&final_path).map_err(|e| e.to_string())?;
        file.write_all(&self.data).map_err(|e| e.to_string())?;
        Ok(final_path)
    }

    fn to_bytes(&self) -> Result<Vec<u8>, String> {
        Ok(self.data.clone())
    }
}

struct PdfWriter {
    buffer: Vec<u8>,
    offsets: Vec<usize>,
}

impl PdfWriter {
    fn new() -> Self {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(b"%PDF-1.4\n");
        // Object 0 is reserved/special in Xref, so we push a dummy offset
        Self {
            buffer,
            offsets: vec![0],
        }
    }

    fn start_obj(&mut self) -> usize {
        let id = self.offsets.len();
        self.offsets.push(self.buffer.len());
        write!(&mut self.buffer, "{} 0 obj\n", id).unwrap();
        id
    }

    fn end_obj(&mut self) {
        self.buffer.extend_from_slice(b"endobj\n");
    }

    fn write(&mut self, s: &str) {
        self.buffer.extend_from_slice(s.as_bytes());
    }

    fn write_bytes(&mut self, b: &[u8]) {
        self.buffer.extend_from_slice(b);
    }

    fn finish(&mut self, root_id: usize) -> Vec<u8> {
        let xref_offset = self.buffer.len();
        self.buffer.extend_from_slice(b"xref\n");
        write!(&mut self.buffer, "0 {}\n", self.offsets.len()).unwrap();
        self.buffer.extend_from_slice(b"0000000000 65535 f \n");
        for offset in self.offsets.iter().skip(1) {
            write!(&mut self.buffer, "{:010} 00000 n \n", offset).unwrap();
        }
        self.buffer.extend_from_slice(b"trailer\n");
        write!(
            &mut self.buffer,
            "<< /Size {} /Root {} 0 R >>\n",
            self.offsets.len(),
            root_id
        )
        .unwrap();
        self.buffer.extend_from_slice(b"startxref\n");
        write!(&mut self.buffer, "{}\n", xref_offset).unwrap();
        self.buffer.extend_from_slice(b"%%EOF\n");
        self.buffer.clone()
    }

    fn create_image_xobject(&mut self, img: &DynamicImage) -> (usize, Option<usize>) {
        let (width, height) = img.dimensions();
        let color_type = img.color();

        let (rgb_data, alpha_data) = match color_type {
            ColorType::Rgba8 | ColorType::Rgba16 | ColorType::Rgba32F => {
                let rgba = img.to_rgba8();
                let mut rgb = Vec::with_capacity((width * height * 3) as usize);
                let mut alpha = Vec::with_capacity((width * height) as usize);
                for pixel in rgba.pixels() {
                    rgb.push(pixel[0]);
                    rgb.push(pixel[1]);
                    rgb.push(pixel[2]);
                    alpha.push(pixel[3]);
                }
                (rgb, Some(alpha))
            }
            _ => {
                let rgb = img.to_rgb8();
                (rgb.into_raw(), None)
            }
        };

        let mut smask_id = None;
        if let Some(alpha) = alpha_data {
            let id = self.start_obj();
            write!(&mut self.buffer, "<< /Type /XObject /Subtype /Image /Width {} /Height {} /ColorSpace /DeviceGray /BitsPerComponent 8 /Length {} >>\n", width, height, alpha.len()).unwrap();
            self.write("stream\n");
            self.write_bytes(&alpha);
            self.write("\nendstream\n");
            self.end_obj();
            smask_id = Some(id);
        }

        let id = self.start_obj();
        write!(&mut self.buffer, "<< /Type /XObject /Subtype /Image /Width {} /Height {} /ColorSpace /DeviceRGB /BitsPerComponent 8 ", width, height).unwrap();
        if let Some(sm_id) = smask_id {
            write!(&mut self.buffer, "/SMask {} 0 R ", sm_id).unwrap();
        }
        write!(&mut self.buffer, "/Length {} >>\n", rgb_data.len()).unwrap();
        self.write("stream\n");
        self.write_bytes(&rgb_data);
        self.write("\nendstream\n");
        self.end_obj();

        (id, smask_id)
    }
}

fn render_pdf<G: QrGrid + ?Sized>(
    grid: &G,
    options: &QrConfig,
    pixel_size: f32,
) -> Result<Vec<u8>, String> {
    let size = grid.size();
    let quiet_zone = options.quiet_zone as f32;
    let width_modules = size as f32 + quiet_zone * 2.0;
    let width_px = width_modules * pixel_size;
    let height_px = width_px;

    let mut writer = PdfWriter::new();

    // 0. Load Icon (if any)
    let mut icon_info = None;
    if let Some(icon_path) = &options.icon {
        if let Some(img) = utils::load_raster_icon(icon_path, "PDF") {
            let (id, _) = writer.create_image_xobject(&img);
            // Calculate size and position
            // Default to 20% of QR size
            let icon_size_px = width_px * 0.25;
            let icon_x = (width_px - icon_size_px) / 2.0;
            let icon_y = (height_px - icon_size_px) / 2.0;
            icon_info = Some((id, icon_size_px, icon_size_px, icon_x, icon_y));
        }
    }

    // 1. Content Stream
    let content_id = writer.start_obj();
    let mut content = String::new();

    // Transform coordinate system: Top-Left origin
    // [1 0 0 -1 0 height]
    use std::fmt::Write as FmtWrite;
    writeln!(&mut content, "1 0 0 -1 0 {:.4} cm", height_px).unwrap();

    // Background
    if let Some(bg) = parse_color(&options.background) {
        writeln!(&mut content, "{:.3} {:.3} {:.3} rg", bg.0, bg.1, bg.2).unwrap();
        writeln!(&mut content, "0 0 {:.4} {:.4} re f", width_px, height_px).unwrap();
    }

    // Save state for QR drawing (to isolate clipping)
    writeln!(&mut content, "q").unwrap();

    // Foreground Color (only if solid)
    if options.foreground.len() <= 1 {
        let fg = if let Some(c) = options.foreground.first().and_then(|c| parse_color(c)) {
            c
        } else {
            (0.0, 0.0, 0.0)
        };
        writeln!(&mut content, "{:.3} {:.3} {:.3} rg", fg.0, fg.1, fg.2).unwrap();
    }

    // Draw modules
    for y in 0..size {
        for x in 0..size {
            if grid.is_dark(x, y) {
                if grid.is_finder(x, y) {
                    continue; // Finders are drawn separately
                }

                let px = (x as f32 + quiet_zone) * pixel_size;
                let py = (y as f32 + quiet_zone) * pixel_size;
                let ctx = grid.module_context(x, y);

                append_module_path(&mut content, options.shape, px, py, pixel_size, &ctx);
            }
        }
    }

    // Finders
    // Top-Left
    append_finder_path(
        &mut content,
        options.finder,
        quiet_zone * pixel_size,
        quiet_zone * pixel_size,
        pixel_size,
    );
    // Top-Right
    append_finder_path(
        &mut content,
        options.finder,
        (size as f32 - 7.0 + quiet_zone) * pixel_size,
        quiet_zone * pixel_size,
        pixel_size,
    );
    // Bottom-Left
    append_finder_path(
        &mut content,
        options.finder,
        quiet_zone * pixel_size,
        (size as f32 - 7.0 + quiet_zone) * pixel_size,
        pixel_size,
    );

    // Fill or Clip
    let mut shading_id = None;

    if options.foreground.len() > 1 {
        // Gradient Mode
        // 1. Clip to the path
        // Use Non-Zero winding rule (W) instead of Even-Odd (W*) to ensure overlapping modules merge
        writeln!(&mut content, "W n").unwrap();

        // 2. Paint Shading
        writeln!(&mut content, "/Sh1 sh").unwrap();

        // We need to create the Shading object later
        shading_id = Some(0); // Placeholder, will be assigned
    } else {
        // Solid Mode
        // Use Non-Zero winding rule (f) instead of Even-Odd (f*)
        writeln!(&mut content, "f").unwrap();
    }

    // Restore state (removes clipping)
    writeln!(&mut content, "Q").unwrap();

    // Draw Icon
    if let Some((_, w, h, x, y)) = icon_info {
        writeln!(&mut content, "q").unwrap();
        // Translate to position, Scale to size, Flip Y (to draw upright in flipped coords)
        // Matrix: w 0 0 -h x (y+h)
        writeln!(
            &mut content,
            "{:.4} 0 0 -{:.4} {:.4} {:.4} cm",
            w,
            h,
            x,
            y + h
        )
        .unwrap();
        writeln!(&mut content, "/Im1 Do").unwrap();
        writeln!(&mut content, "Q").unwrap();
    }

    writer.write(&format!("<< /Length {} >>\n", content.len()));
    writer.write("stream\n");
    writer.write(&content);
    writer.write("endstream\n");
    writer.end_obj();

    // 2. Shading Resources (if needed)
    let mut resources_str = String::new();
    let mut shading_res = String::new();

    if let Some(_) = shading_id {
        let colors: Vec<(f32, f32, f32)> = options
            .foreground
            .iter()
            .filter_map(|c| parse_color(c))
            .collect();

        if !colors.is_empty() {
            // Function Object
            let func_id = writer.start_obj();
            writer.write(&utils::generate_pdf_ps_gradient_function(&colors));
            writer.write("\n");
            writer.end_obj();

            // Shading Object
            let sh_id = writer.start_obj();
            let (x0, y0, r0, x1, y1, r1) =
                utils::get_gradient_coords(options.gradient_direction, width_px, height_px);
            let shading_type = if options.gradient_direction == GradientDirection::Radial {
                3
            } else {
                2
            };

            writer.write(&format!(
                "<< /ShadingType {} /ColorSpace /DeviceRGB ",
                shading_type
            ));
            if shading_type == 2 {
                writer.write(&format!(
                    "/Coords [ {:.3} {:.3} {:.3} {:.3} ] ",
                    x0, y0, x1, y1
                ));
            } else {
                writer.write(&format!(
                    "/Coords [ {:.3} {:.3} {:.3} {:.3} {:.3} {:.3} ] ",
                    x0, y0, r0, x1, y1, r1
                ));
            }
            writer.write(&format!(
                "/Function {} 0 R /Extend [ true true ] >>\n",
                func_id
            ));
            writer.end_obj();

            shading_res = format!(" /Shading << /Sh1 {} 0 R >>", sh_id);
        }
    }

    let mut xobject_res = String::new();
    if let Some((id, _, _, _, _)) = icon_info {
        xobject_res = format!(" /XObject << /Im1 {} 0 R >>", id);
    }

    if !shading_res.is_empty() || !xobject_res.is_empty() {
        resources_str = format!(
            "<<{}{}{} >>",
            if !shading_res.is_empty() {
                shading_res.as_str()
            } else {
                ""
            },
            if !xobject_res.is_empty() {
                xobject_res.as_str()
            } else {
                ""
            },
            "" // Just to handle trailing space if needed, but format handles it
        );
    }

    // 3. Page
    let page_id = writer.start_obj();
    writer.write(&format!(
        "<< /Type /Page /Parent {} 0 R /MediaBox [0 0 {:.4} {:.4}] /Contents {} 0 R ",
        page_id + 1,
        width_px,
        height_px,
        content_id
    )); // Parent will be next object
    if !resources_str.is_empty() {
        writer.write(&format!("/Resources {} ", resources_str));
    }
    writer.write(">>\n");
    writer.end_obj();

    // 4. Pages
    let pages_id = writer.start_obj();
    writer.write(&format!(
        "<< /Type /Pages /Kids [{} 0 R] /Count 1 >>\n",
        page_id
    ));
    writer.end_obj();

    // 5. Catalog
    let catalog_id = writer.start_obj();
    writer.write(&format!("<< /Type /Catalog /Pages {} 0 R >>\n", pages_id));
    writer.end_obj();

    Ok(writer.finish(catalog_id))
}

fn parse_color(hex: &str) -> Option<(f32, f32, f32)> {
    utils::parse_hex_color(hex)
        .map(|(r, g, b)| (r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0))
}
