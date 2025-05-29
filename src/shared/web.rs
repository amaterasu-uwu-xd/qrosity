use clap::Args;

use super::SharedArgs;

#[derive(Args)]
pub struct Web {
    /// The URL to encode
    #[arg(long, value_hint = clap::ValueHint::Url, value_parser = verify_url)]
    pub url: String,

    #[clap(flatten)]
    pub shared: SharedArgs,
}

fn verify_url(url: &str) -> Result<String, String> {
    regex::Regex::new(r"https?://[^\s/?#]+(?:/[^\s?#]*)?(?:\?[^\s#]*)?(?:#[^\s]*)?")
        .map_err(|_| "Invalid regex".to_string())?
        .is_match(url)
        .then_some(url.to_string())
        .ok_or_else(|| "Invalid URL format".to_string())
}

impl Web {
    /// Struct the data to use it in the QR
    pub fn to_str(&self) -> String {
        self.url.clone()
    }

    /// This will be used to the interactive mode
    pub fn get_data() -> Web {
        todo!()
    }
}
