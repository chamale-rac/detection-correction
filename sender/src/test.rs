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
        .expect("Failed to open file");

    let mut wtr = Writer::from_writer(file);

    println!("Automated Test Started...");

    let error_rates = [0.1, 1.0, 2.0, 3.0, 4.0, 5.0];
    let bit_lengths = (1..=10).map(|x| 2_usize.pow(x)).collect::<Vec<_>>(); // 2,4,8,16,32,64

    wtr.write_record(&[
        "Error Rate",
        "Length",
        "Original",
        "Encoded",
        "Noisy",
        "Has Errors",
    ])
    .expect("Failed to write header");

    for &error_rate in &error_rates {
        for &length in &bit_lengths {
            let message = generate_random_message(length);
            println!("Length: {}", message.len());

            let binary_message = encode_message(&message);
            let generator = "100000100110000010001110110110111"; // CRC32 generator polynomial
            let crc_message = calculate_crc(&binary_message, generator);
            let (noisy_message, has_errors) = apply_noise(&crc_message, error_rate as f64 / 100.0); // Adjust error rate to fraction

            wtr.write_record(&[
                &error_rate.to_string(),
                &message.len().to_string(),
                &message,
                &crc_message,
                &noisy_message, // What the receiver will get
                &has_errors.to_string(),
            ])
            .expect("Failed to write record");
        }
    }

    wtr.flush().expect("Failed to flush writer");

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

// Modify the apply_noise function to return a tuple (noisy_message, has_errors)
fn apply_noise(message: &str, error_rate: f64) -> (String, bool) {
    let mut noisy_message = message.to_string();
    let mut has_errors = false;
    for i in 0..message.len() {
        if rand::random::<f64>() < error_rate {
            noisy_message.replace_range(i..=i, if &message[i..=i] == "0" { "1" } else { "0" });
            has_errors = true;
        }
    }
    (noisy_message, has_errors)
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
