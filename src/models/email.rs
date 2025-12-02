use std::fmt;

#[cfg(feature = "cli")]
use clap::Args;

#[derive(Debug)]
#[cfg_attr(feature = "cli", derive(Args))]
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
