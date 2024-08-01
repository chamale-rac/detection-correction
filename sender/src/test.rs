use crate::crc32;
use crate::hamming;
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

pub fn generate_hamming_test_cases() {
    use hamming::*;

    env::set_var("RUST_BACKTRACE", "1");
    use std::fs::OpenOptions;

    let file_path = "../../tests/test_cases_hamming.csv";
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(file_path)
        .expect("Failed to open file");

    let mut wtr = Writer::from_writer(file);

    println!("Automated Test Started...");

    let error_rates = [0.1, 0.2, 0.3, 0.4, 0.5];
    let bit_lengths = (1..=10).map(|x| 2_usize.pow(x)).collect::<Vec<_>>(); // 2,4,8,16,32,64

    wtr.write_record(&[
        "Error Rate",
        "Length",
        "Original",
        "Encoded",
        "Noisy",
        "Errors",
    ])
    .expect("Failed to write header");

    let n = 7; // Number of parity bits
    let m = 4; // Number of data bits

    for &error_rate in &error_rates {
        for &length in &bit_lengths {
            let message = generate_random_message(length);
            let binary_message = encode_message(&message);

            let padded_binary_message = if binary_message.len() % m != 0 {
                println!("Alert: The message length is not a multiple of m.");
                let padded_message = pad_message(binary_message.clone(), m);
                println!("- The padded message is:\n{}", padded_message);
                padded_message
            } else {
                binary_message.clone()
            };

            let mut complete_hamming_code = String::new();
            for i in 0..padded_binary_message.len() / m {
                let block = &padded_binary_message[i * m..(i + 1) * m];
                let hamming_code = generate_hamming_code(block, n - m);
                complete_hamming_code.push_str(&hamming_code);
            }

            // size of original message
            println!("Length: {}", message.len());

            // size of encoded message
            println!("Encoded Length: {}", complete_hamming_code.len());

            let (noisy_message, error_count) = apply_noise(&complete_hamming_code, error_rate);

            wtr.write_record(&[
                &error_rate.to_string(),
                &message.len().to_string(),
                &message,
                &complete_hamming_code,
                &noisy_message, // What the receiver will get
                &error_count.to_string(),
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

fn apply_noise(message: &str, error_rate: f64) -> (String, i32) {
    let mut noisy_message = String::new();
    let mut error_count = 0;

    for bit in message.chars() {
        if rand::random::<f64>() < error_rate {
            let flipped_bit = if bit == '0' { '1' } else { '0' };
            noisy_message.push(flipped_bit);
            error_count += 1;
        } else {
            noisy_message.push(bit);
        }
    }

    (noisy_message, error_count)
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
