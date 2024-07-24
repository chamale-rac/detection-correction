use std::env;
use std::io::{self, Write};

// fn calculate_redundant_bits(m: usize) -> usize {
//     // For using auto generated r
//     let mut r = 0;
//     while !validate_redundancy_bits(m, r) {
//         r += 1;
//     }
//     r
// }

fn validate_redundancy_bits(n: usize, r: usize) -> bool {
    // The formula to check if r is sufficient for m data bits is 2^r >= m + r + 1
    2usize.pow(r as u32) >= n + r + 1
}

fn validate_is_binary(message: &str) -> bool {
    message.chars().all(|c| c == '0' || c == '1')
}

fn generate_hamming_code(message: &str, r: usize) -> String {
    let total_bits = message.len() + r;
    let mut hamming_code = vec!['0'; total_bits];

    // Place data bits
    let mut j = 0;
    for i in 1..=total_bits {
        if i & (i - 1) != 0 {
            // Check if i is not a power of 2
            hamming_code[i - 1] = message.chars().nth(j).unwrap();
            j += 1;
        }
    }

    // Calculate parity bits
    for i in 0..r {
        let parity_position = 1 << i; // 2^i
        let mut parity = 0;
        for j in 1..=total_bits {
            if j & parity_position != 0 {
                // bits positions whose binary representation includes a 1 in the i-th position
                parity ^= hamming_code[j - 1].to_digit(10).unwrap(); // Assuming hamming_code contains '0' or '1'
            }
        }
        hamming_code[parity_position - 1] = if parity == 1 { '1' } else { '0' };
    }
    // Based on https://en.wikipedia.org/wiki/Hamming_code General algorithm

    hamming_code.into_iter().collect()
}

fn pad_message(mut message: String, block_size: usize) -> String {
    let original_length = message.len();
    let padding_length = block_size - (original_length % block_size);

    while message.len() < original_length + padding_length {
        message.push('0');
    }

    message
}

fn main() {
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

    // println!("Splitting the message into blocks of m bits.");

    let mut complete_hamming_code = String::new();

    for i in 0..message.len() / m {
        let block = &message[i * m..(i + 1) * m];
        let hamming_code = generate_hamming_code(block, n - m);
        // print i, and hamming code
        // println!("{} : {}", hamming_code, i + 1);
        complete_hamming_code.push_str(&hamming_code);
    }

    println!("The hamming code is: {}", complete_hamming_code);
}
