use std::fmt;
use super::{QrConfig, QrItem};

#[cfg(feature = "cli")]
use clap::Args;

/// Represents the data needed to generate a QR code for an email.
/// This includes the recipient's email address, subject, body, and optional CC and BCC recipients.
#[derive(Debug)]
#[cfg_attr(feature = "cli", derive(Args))]
#[cfg_attr(feature = "batch", derive(serde::Serialize, serde::Deserialize))]
pub struct EmailQr {
    #[cfg_attr(feature = "cli", arg(long, help = "Recipient email address"))]
    pub to: String,

    #[cfg_attr(feature = "cli", arg(long, help = "Email subject"))]
    pub subject: Option<String>,

    #[cfg_attr(feature = "cli", arg(long, help = "Email body"))]
    pub body: Option<String>,

    #[cfg_attr(feature = "cli", arg(long, help = "CC recipient"))]
    pub cc: Option<String>,

    #[cfg_attr(feature = "cli", arg(long, help = "BCC recipient"))]
    pub bcc: Option<String>,

    #[cfg_attr(feature = "cli", command(flatten))]
    #[cfg_attr(feature = "batch", serde(flatten))]
    pub config: QrConfig,
}

impl fmt::Display for EmailQr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();

        if let Some(subject) = &self.subject {
            parts.push(format!("subject={}", urlencoding::encode(subject)));
        }
        if let Some(body) = &self.body {
            parts.push(format!("body={}", urlencoding::encode(body)));
        }
        if let Some(cc) = &self.cc {
            parts.push(format!("cc={}", urlencoding::encode(cc)));
        }
        if let Some(bcc) = &self.bcc {
            parts.push(format!("bcc={}", urlencoding::encode(bcc)));
        }

        write!(f, "mailto:{}", self.to)?;
        
        if !parts.is_empty() {
            write!(f, "?{}", parts.join("&"))?;
        }

        Ok(())
    }
}

impl QrItem for EmailQr {
    fn config(&self) -> &QrConfig {
        &self.config
    }
}
