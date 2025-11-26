#[cfg(feature = "cli")]
use clap::Args;

#[derive(Debug)]
#[cfg_attr(feature = "cli", derive(Args))]
pub struct TextQr {
    #[cfg_attr(feature = "cli", arg(long, help = "Text to encode in the QR code"))]
    pub input: Option<String>,
}

impl std::fmt::Display for TextQr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(input) = &self.input {
            write!(f, "{}", input)
        } else {
            Ok(())
        }
    }
}
