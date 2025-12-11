mod email;
mod text;
mod wifi;

mod config;
mod image;
mod wrapper;

pub use config::FinderShape;
pub use config::GradientDirection;
pub use config::ModuleShape;
pub use config::OutputFormat;
pub use config::QrConfig;
pub use email::EmailQr;
pub use image::QrImage;
pub use text::TextQr;
pub use wifi::WifiQr;
pub use wrapper::QrData;

/// Trait representing a QR code item that can provide its configuration
/// for rendering and can be displayed as a string.
/// Implementors of this trait must also implement the Display trait.
/// Useful if you want to add more QR code types.
pub trait QrItem: std::fmt::Display {
    fn config(&self) -> &QrConfig;
}
