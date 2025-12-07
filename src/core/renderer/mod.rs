use crate::core::qrgen::QrCode;

pub mod png;

#[cfg(feature = "svg")]
pub mod svg;

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
}

impl QrGrid for QrCode {
    fn size(&self) -> usize {
        self.size() as usize
    }

    fn get_module(&self, x: usize, y: usize) -> bool {
        self.get_module(x as i32, y as i32)
    }
}
