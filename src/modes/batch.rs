use crate::models::{QrData, QrItem};
use crate::core::to_qr;
use std::fs::File;
use std::io::BufReader;
use rayon::prelude::*;

/// Runs the batch processing mode.
/// Reads a JSON file containing multiple QR code data items and generates QR codes for each.
pub fn run(input_path: String, threads: usize) {
    let file = match File::open(&input_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file '{}': {}", input_path, e);
            return;
        }
    };
    let reader = BufReader::new(file);

    let items: Vec<QrData> = match serde_json::from_reader(reader) {
        Ok(items) => items,
        Err(e) => {
            eprintln!("Error parsing JSON: {}", e);
            return;
        }
    };

    println!("Processing {} items with {} threads...", items.len(), threads);

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .unwrap();

    pool.install(|| {
        items.into_par_iter().for_each(|item| {
            let content = item.to_string();
            if content.is_empty() {
                eprintln!("Skipping item with empty content (output: {})", item.config().output);
                return;
            }
            to_qr(item);
        });
    });
}

