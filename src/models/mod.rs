mod wifi;
mod text;
mod email;

mod config;
mod wrapper;

pub use wifi::WifiQr;
pub use text::TextQr;
pub use email::EmailQr;
pub use wrapper::QrData;
pub use config::QrConfig;
pub use config::FinderShape;
pub use config::ModuleShape;
pub use config::GradientDirection;

/// Trait representing a QR code item that can provide its configuration
/// for rendering and can be displayed as a string.
/// Implementors of this trait must also implement the Display trait.
/// Useful if you want to add more QR code types.
pub trait QrItem: std::fmt::Display {
    fn config(&self) -> &QrConfig;
}
