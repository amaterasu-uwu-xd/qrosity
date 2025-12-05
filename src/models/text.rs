use super::QrConfig;

#[cfg(feature = "cli")]
use clap::Args;

#[derive(Debug)]
#[cfg_attr(feature = "cli", derive(Args))]
#[cfg_attr(feature = "batch", derive(serde::Serialize, serde::Deserialize))]
pub struct TextQr {
    #[cfg_attr(feature = "cli", arg(help = "Text to encode in the QR code"))]
    pub text: Option<String>,

    #[cfg_attr(feature = "cli", command(flatten))]
    #[cfg_attr(feature = "batch", serde(flatten))]
    pub config: QrConfig,
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
