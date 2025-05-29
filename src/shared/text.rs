use clap::Args;
use crate::commands::cli::SharedArgs;

#[derive(Args)]
pub struct Text {
    /// Text to encode in the QR code
    #[arg(long, required = true)]
    pub text: String,

    #[clap(flatten)]
    pub shared: SharedArgs,
}

impl super::Qr for Text {
    fn to_str(&self) -> String {
        self.text.clone()
    }

    fn get_data<T: super::Qr>() -> T {
        todo!()
    }
}