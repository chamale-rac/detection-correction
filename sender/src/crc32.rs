// Function to convert a binary string to an integer array
fn binary_string_to_int_array(binary: &str) -> Vec<u8> {
    binary
        .chars()
        .map(|c| if c == '1' { 1 } else { 0 })
        .collect()
}

// Function to convert an integer array to a binary string
fn int_array_to_binary_string(arr: &[u8]) -> String {
    arr.iter()
        .map(|&val| if val == 1 { '1' } else { '0' })
        .collect()
}

// Function to perform modulo-2 division and return the remainder
fn mod2_division(dividend: &[u8], divisor: &[u8]) -> Vec<u8> {
    let mut dividend_copy = dividend.to_vec();

    // Using an unitary and fixed displacement is easier than using logic to find the correct position
    for i in 0..(dividend.len() - divisor.len() + 1) {
        // We still optimize the code by not performing the XOR operation if the dividend_copy[i] is 0
        if dividend_copy[i] == 1 {
            for j in 0..divisor.len() {
                dividend_copy[i + j] ^= divisor[j]; // XOR operation
            }
        }
    }
    dividend_copy[(dividend.len() - divisor.len() + 1)..].to_vec() // Return the remainder
}

// Function to calculate the CRC of a frame
pub fn calculate_crc(frame: &str, generator: &str) -> String {
    let mut frame_bits = binary_string_to_int_array(frame);
    let generator_bits = binary_string_to_int_array(generator);

    // Append zero bits: frame + generator
    frame_bits.extend(vec![0; generator_bits.len() - 1]);

    // Perform modulo-2 division
    let remainder = mod2_division(&frame_bits, &generator_bits);

    // Subtract the remainder from the appended zero bits frame
    for i in (frame_bits.len() - remainder.len())..frame_bits.len() {
        frame_bits[i] ^= remainder[i - (frame_bits.len() - remainder.len())];
    }

    int_array_to_binary_string(&frame_bits)
}

// Every concept get from https://csc-knu.github.io/sys-prog/books/Andrew%20S.%20Tanenbaum%20-%20Computer%20Networks.pdf
// Pages 212-215 Page 21
// Some other concepts obtained from http://www.sunshine2k.de/articles/coding/crc/understanding_crc.html

// fn main() {
//     let frame = "1101011111";
//     // IEEE 802: x^{32} + x^{26} + x^{23} + x^{22} + x^{16} + x^{12} + x^{11} + x^{10} + x^8 + x^7 + x^5 + x^4 + x^2 + x^1 + 1
//     // Page 215 https://csc-knu.github.io/sys-prog/books/Andrew%20S.%20Tanenbaum%20-%20Computer%20Networks.pdf
//     let generator = "100000100110000010001110110110111";

//     // Book example
//     // let frame = "1101011111";
//     // let generator = "10011";

//     // Class example
//     // let frame = "11010";
//     // let generator = "1001";

//     let crc = calculate_crc(frame, generator);

//     println!("Transmitted frame: {}", crc);
// }
