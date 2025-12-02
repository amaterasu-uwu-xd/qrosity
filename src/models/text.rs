#[cfg(feature = "cli")]
use clap::Args;

#[derive(Debug)]
#[cfg_attr(feature = "cli", derive(Args))]
pub struct TextQr {
    #[cfg_attr(feature = "cli", arg(help = "Text to encode in the QR code"))]
    pub text: Option<String>,
}

impl std::fmt::Display for TextQr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(text) = &self.text {
            write!(f, "{}", text)
        } else {
            Ok(())
        }
    }
}
