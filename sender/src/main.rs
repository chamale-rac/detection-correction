use csv::Writer;
use rand::Rng;
use std::env;
use std::io::{self, Write};
use std::net::TcpStream;
mod crc32;
mod hamming;
mod test;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("--------------------------------------");
        println!("\t📨 Sender started...");
        println!("--------------------------------------");
        println!("1. Hamming Code");
        println!("2. CRC32");
        print!("Choose an option (1 or 2): ");
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
            "crc32_test" => hamming_test(),
            _ => {
                eprintln!("Invalid choice: {}. Use 'hamming' or 'crc32'.", choice);
            }
        }
    }
}

fn hamming_test() {
    use test::generate_test_cases;
    generate_test_cases();
}

fn run_crc32() {
    use crc32::calculate_crc;
    env::set_var("RUST_BACKTRACE", "1");

    println!("::: CRC32");

    let generator = "100000100110000010001110110110111"; // Check the crc32.rs for examples

    println!("Enter the message to send: ");
    let message = get_input();

    println!("⚒️  Starting CRC32 encoding...");

    let binary_message = encode_message(&message);
    println!("- Encoded to binary:\n{}", binary_message);

    let crc_message = calculate_crc(&binary_message, generator);
    println!("- CRC32 encoded message:\n{}", crc_message);

    let error_rate = get_error_rate();

    println!("🔊 Applying {}% noise...", error_rate);
    let noisy_message = apply_noise(&crc_message, error_rate);

    println!("- With noise applied:\n{}", noisy_message);

    let address = get_address();

    if let Err(e) = send_message(&address, &noisy_message) {
        println!("❌ Failed to send message:\n{}", e);
    } else {
        println!("✅ Message sent successfully.");
    }
}

fn run_hamming() {
    use hamming::*;
    env::set_var("RUST_BACKTRACE", "1");

    println!("::: Hamming Code");

    let n = get_n_value();
    let m = get_m_value();

    if !validate_redundancy_bits(n, m) {
        println!("The number of redundancy bits is not sufficient.");
        return;
    }

    println!("Enter the message to send: ");
    let message = get_input();

    println!("⚒️  Starting Hamming encoding...");

    let binary_message = encode_message(&message);
    println!("- Encoded to binary:\n{}", binary_message);

    // Check the message is a multiple of m and pad if necessary
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
    println!("- Hamming encoded message:\n{}", complete_hamming_code);

    let error_rate = get_error_rate();

    println!("🔊 Applying {}% noise...", error_rate);
    let noisy_message = apply_noise(&complete_hamming_code, error_rate);

    println!("- With noise applied:\n{}", noisy_message);

    let address = get_address();

    if let Err(e) = send_message(&address, &noisy_message) {
        println!("❌ Failed to send message:\n{}", e);
    } else {
        println!("✅ Message sent successfully.");
    }
}

// Application Layer: Get user input for the message
fn get_input() -> String {
    print!("> ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
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

// Utility function: Get error rate from the user
fn get_error_rate() -> f64 {
    println!("Enter the error rate (errors per 100 bits): ");
    print!("> ");
    io::stdout().flush().unwrap();
    let mut error_rate_str = String::new();
    io::stdin().read_line(&mut error_rate_str).unwrap();
    error_rate_str.trim().parse().unwrap()
}

// Utility function: Get address from the user
fn get_address() -> String {
    println!("Enter the address (e.g., localhost:12345): ");
    print!("> ");
    io::stdout().flush().unwrap();
    let mut address = String::new();
    io::stdin().read_line(&mut address).unwrap();
    address.trim().to_string()
}

// Transmission Layer: Send message via socket
fn send_message(address: &str, message: &str) -> io::Result<()> {
    let mut stream = TcpStream::connect(address)?;
    stream.write_all(message.as_bytes())?;
    stream.write_all(b"\n")?;
    Ok(())
}

// Utility function: Get n value from the user
fn get_n_value() -> usize {
    print!("Number of bits in a block (n): ");
    io::stdout().flush().unwrap();
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    n.trim().parse().unwrap()
}

// Utility function: Get m value from the user
fn get_m_value() -> usize {
    print!("Number of data bits in a block (m): ");
    io::stdout().flush().unwrap();
    let mut m = String::new();
    io::stdin().read_line(&mut m).unwrap();
    m.trim().parse().unwrap()
}
