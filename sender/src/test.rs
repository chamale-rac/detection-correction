use crate::crc32;
use csv::Writer;
use rand::Rng;
use std::env;
use std::io::{self, Write};

pub fn generate_test_cases() {
    env::set_var("RUST_BACKTRACE", "1");
    use crc32::*;
    use std::fs::OpenOptions;

    let file_path = "../../tests/test_cases_crc32.csv";
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(file_path)
        .unwrap();

    let mut wtr = Writer::from_writer(file);

    println!("Automated Test Started...");

    let error_rates = [1, 2, 5, 10];

    wtr.write_record(&["Error Rate", "Length", "Original", "Encoded", "Noisy"])
        .unwrap();

    for &error_rate in &error_rates {
        let mut bits = 4; // Reset bits for each error rate
        while bits <= 64 {
            let message = generate_random_message(bits);
            let binary_message = encode_message(&message);
            let generator = "100000100110000010001110110110111"; // CRC32 generator polynomial
            let crc_message = calculate_crc(&binary_message, generator);
            let noisy_message = apply_noise(&crc_message, error_rate as f64 / 100.0);

            wtr.write_record(&[
                &error_rate.to_string(),
                &message.len().to_string(),
                &message,
                &crc_message,
                &noisy_message, // What the receiver will get
            ])
            .unwrap();

            bits += 4;
        }
    }

    wtr.flush().unwrap();

    println!("Automated Test Completed. Results saved to {}.", file_path);
}

// Presentation Layer: Encode message to binary ASCII
fn encode_message(message: &str) -> String {
    message
        .chars()
        .map(|c| format!("{:08b}", c as u8))
        .collect::<Vec<String>>()
        .concat()
}

// Noise Layer: Apply noise to the binary message
fn apply_noise(binary_message: &str, error_rate: f64) -> String {
    let mut rng = rand::thread_rng();
    binary_message
        .chars()
        .enumerate() // Add enumerate to get the position
        .map(|(pos, bit)| {
            if rng.gen::<f64>() < error_rate / 100.0 {
                if bit == '0' {
                    println!("Error introduced at position {}, flipped 0 to 1", pos);
                    '1'
                } else {
                    println!("Error introduced at position {}, flipped 1 to 0", pos);
                    '0'
                }
            } else {
                bit
            }
        })
        .collect()
}

fn get_error_rate() -> f64 {
    println!("Enter the error rate (errors per 100 bits): ");
    print!("> ");
    io::stdout().flush().unwrap();
    let mut error_rate_str = String::new();
    io::stdin().read_line(&mut error_rate_str).unwrap();
    error_rate_str.trim().parse().unwrap()
}

fn calculate_errors(original: &str, noisy: &str) -> usize {
    original
        .chars()
        .zip(noisy.chars())
        .filter(|(o, n)| o != n)
        .count()
}

fn generate_random_message(bits: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..bits)
        .map(|_| if rng.gen::<bool>() { '1' } else { '0' })
        .collect()
}
