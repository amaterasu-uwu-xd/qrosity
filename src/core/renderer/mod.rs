use crate::core::qrgen::QrCode;

pub mod png;

#[cfg(feature = "svg")]
pub mod svg;

pub struct ModuleContext {
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
}

// Asumimos que tienes un trait o struct para tu matriz
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
