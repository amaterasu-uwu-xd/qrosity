use clap::Args;

use super::Qr;

#[derive(Args)]
pub struct Mail {
    /// Email address
    #[arg(long, required = true, value_parser = validate_email)]
    pub email: String,

    /// Subject of the email
    #[arg(long)]
    pub subject: Option<String>,

    /// Body of the email
    #[arg(long)]
    pub body: Option<String>,

    #[clap(flatten)]
    pub shared: super::SharedArgs,
}

fn validate_email(email: &str) -> Result<String, String> {
    regex::Regex::new(r"^[\w\-\.]+@([\w-]+\.)+[\w-]{2,}$")
        .map_err(|_| "Invalid regex".to_string())?
        .is_match(email)
        .then_some(email.to_string())
        .ok_or_else(|| "Invalid email format".to_string())
}

impl Qr for Mail {
    fn to_str(&self) -> String {
        format!(
            "mailto:{}{}{}",
            self.email,
            self.subject.as_ref().map_or(String::new(), |s| format!("?subject={}", s)),
            self.body.as_ref().map_or(String::new(), |b| format!("&body={}", b))
        )
    }

    fn get_data<T: Qr>() -> T {
        todo!()
    }
    
}