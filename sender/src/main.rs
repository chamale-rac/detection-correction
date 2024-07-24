use std::env;
use std::io::{self, Write};

mod crc32;
mod hamming;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("No function specified. Please choose a function to run:");
        println!("1. Hamming Code");
        println!("2. CRC32 (not yet implemented)");
        print!("Enter your choice (1 or 2): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => run_hamming(),
            "2" => run_crc32(),
            _ => {
                eprintln!("Invalid choice: {}. Please enter 1 or 2.", choice);
            }
        }
    } else {
        let choice = &args[1];

        match choice.as_str() {
            "hamming" => run_hamming(),
            "crc32" => run_crc32(),
            _ => {
                eprintln!("Invalid choice: {}. Use 'hamming' or 'crc32'.", choice);
            }
        }
    }
}

fn run_crc32() {
    use crc32::calculate_crc;
    env::set_var("RUST_BACKTRACE", "1");

    // IEEE 802: x^{32} + x^{26} + x^{23} + x^{22} + x^{16} + x^{12} + x^{11} + x^{10} + x^8 + x^7 + x^5 + x^4 + x^2 + x^1 + 1
    // Page 215 https://csc-knu.github.io/sys-prog/books/Andrew%20S.%20Tanenbaum%20-%20Computer%20Networks.pdf
    let generator = "100000100110000010001110110110111"; // Check the crc23.rs for examples

    print!("Enter the binary message: ");
    io::stdout().flush().unwrap();
    let mut frame = String::new();
    io::stdin().read_line(&mut frame).unwrap();
    let frame = frame.trim();

    if !validate_is_binary(frame) {
        println!("The message is not binary.");
        return;
    }

    let crc = calculate_crc(frame, generator);

    println!("Transmitted message: {}", crc);
}

fn run_hamming() {
    use hamming::*;
    env::set_var("RUST_BACKTRACE", "1");

    print!(">> (n): ");
    io::stdout().flush().unwrap();
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();

    print!(">> (m): ");
    io::stdout().flush().unwrap();
    let mut m = String::new();
    io::stdin().read_line(&mut m).unwrap();
    let m: usize = m.trim().parse().unwrap();

    if !validate_redundancy_bits(n, m) {
        println!("The number of redundancy bits is not sufficient.");
        return;
    }

    println!("The number of redundancy bits is sufficient.");

    print!("Enter the binary message: ");
    io::stdout().flush().unwrap();
    let mut message = String::new();
    io::stdin().read_line(&mut message).unwrap();
    let mut message = message.trim().to_string(); // Change the message to String type

    if !validate_is_binary(&message) {
        println!("The message is not binary.");
        return;
    }

    // Check the message is a multiple of m
    if message.len() % m != 0 {
        println!("The message length is not a multiple of m.");

        message = pad_message(message, m); // Directly assign the padded message

        println!("The padded message is: {}", message);
    }

    let mut complete_hamming_code = String::new();

    for i in 0..message.len() / m {
        let block = &message[i * m..(i + 1) * m];
        let hamming_code = generate_hamming_code(block, n - m);
        complete_hamming_code.push_str(&hamming_code);
    }

    println!("The hamming code is: {}", complete_hamming_code);
}

fn validate_is_binary(message: &str) -> bool {
    message.chars().all(|c| c == '0' || c == '1')
}
