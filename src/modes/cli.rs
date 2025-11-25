use crate::{core::to_qr, models::{ QrConfig, QrData }};

pub fn run(config: QrConfig, data: QrData) {
    println!("String: {}", data);

    to_qr(data.to_string(), config);
}