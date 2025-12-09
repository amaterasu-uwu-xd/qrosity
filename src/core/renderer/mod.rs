use crate::core::qrgen::QrCode;

pub mod png;
pub mod svg;
pub mod eps;

/// Trait for QR code renderers.
/// Allows rendering to an in-memory format and saving to a file.
pub trait QrRenderer {
    fn save(&self, path: &str) -> Result<String, String>;
}

/// Provides context about the position of a module in the QR code.
/// This is useful for rendering purposes, such as determining
/// if a module is at the edge of the QR code.
pub struct ModuleContext {
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
}

/// Trait representing a QR code grid for rendering purposes.
pub trait QrGrid {
    fn size(&self) -> usize;
    fn get_module(&self, x: usize, y: usize) -> bool;
    fn is_dark(&self, x: usize, y: usize) -> bool {
        self.get_module(x, y)
    }
    fn is_finder(&self, x: usize, y: usize) -> bool {
        if x < 7 && y < 7 { return true; }
        if x >= self.size() - 7 && y < 7 { return true; }
        if x < 7 && y >= self.size() - 7 { return true; }
        false
    }
    fn module_context(&self, x: usize, y: usize) -> ModuleContext {
        ModuleContext {
            top: y > 0 && self.get_module(x, y - 1),
            bottom: y < self.size() - 1 && self.get_module(x, y + 1),
            left: x > 0 && self.get_module(x - 1, y),
            right: x < self.size() - 1 && self.get_module(x + 1, y),
        }
    }
}

impl QrGrid for QrCode {
    fn size(&self) -> usize {
        self.size() as usize
    }

    fn get_module(&self, x: usize, y: usize) -> bool {
        self.get_module(x as i32, y as i32)
    }
}
