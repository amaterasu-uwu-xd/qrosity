use crate::core::generate_qr;
use crate::models::QrData;
use rayon::prelude::*;
use std::fs::File;
use std::io::BufReader;

#[derive(serde::Deserialize)]
struct BatchItem {
    #[serde(flatten)]
    data: QrData,
    output: String,
}

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

    let items: Vec<BatchItem> = match serde_json::from_reader(reader) {
        Ok(items) => items,
        Err(e) => {
            eprintln!("Error parsing JSON: {}", e);
            return;
        }
    };

    println!(
        "Processing {} items with {} threads...",
        items.len(),
        threads
    );

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .unwrap();

    pool.install(|| {
        items.into_par_iter().for_each(|item| {
            let output = item.output.unwrap_or_else(|| {
                format!("qr_{}", chrono::Local::now().format("%Y-%m-%d_%H-%M-%S_%f"))
            });

            match generate_qr(&item.data) {
                Ok(renderer) => {
                    if let Err(e) = renderer.save(&output) {
                        eprintln!("Error saving {}: {}", output, e);
                    } else {
                        println!("Saved {}", output);
                    }
                }
                Err(e) => eprintln!("Error generating QR: {}", e),
            }
        });
    });
}
