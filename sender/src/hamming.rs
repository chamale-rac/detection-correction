use num_bigint::BigUint;
use num_traits::One;

pub fn validate_redundancy_bits(n: usize, r: usize) -> bool {
    let n_big = BigUint::from(n);
    let r_big = BigUint::from(r);
    let two = BigUint::from(2u32);
    let power_of_two = two.pow(r as u32);
    power_of_two >= n_big + r_big + BigUint::one()
}

pub fn validate_is_binary(message: &str) -> bool {
    message.chars().all(|c| c == '0' || c == '1')
}

pub fn generate_hamming_code(message: &str, r: usize) -> String {
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

pub fn pad_message(mut message: String, block_size: usize) -> String {
    let original_length = message.len();
    let padding_length = block_size - (original_length % block_size);

    while message.len() < original_length + padding_length {
        message.push('0');
    }

    message
}
