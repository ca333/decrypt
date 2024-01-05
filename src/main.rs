use std::env;

/// Decrypts a text encrypted with a Caesar cipher.
///
/// # Arguments
/// * `text` - The encrypted text to decrypt.
/// * `shift` - The shift used in the Caesar cipher.
/// 
fn decrypt_caesar_cipher(text: &str, shift: i32) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                // Determine base character for case sensitivity
                let first_char = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                // Calculate and apply the shift
                let offset = c as i32 - first_char as i32;
                // Adjust shift for negative values
                let adjusted_shift = (shift % 26 + 26) % 26;
                let shifted = (offset - adjusted_shift + 26) % 26;
                (first_char as i32 + shifted) as u8 as char
            } else {
                // Non-alphabetic characters are left unchanged
                c
            }
        })
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Ensure correct usage with two arguments
    if args.len() != 3 {
        eprintln!("Usage: {} <encrypted_text> <shift>", args[0]);
        std::process::exit(1);
    }

    let encrypted_text = &args[1];

    // Handle invalid shift values and negative shifts
    let shift = match args[2].parse::<i32>() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: Shift must be a number");
            std::process::exit(1);
        },
    };

    // Check for empty input
    if encrypted_text.is_empty() {
        eprintln!("Error: Encrypted text cannot be empty");
        std::process::exit(1);
    }

    let decrypted_text = decrypt_caesar_cipher(encrypted_text, shift);
    println!("Decrypted Text: {}", decrypted_text);
}
