use std::env;

/// Decrypts a text encrypted with a Caesar cipher.
///
/// # Arguments
/// * `text` - The encrypted text to decrypt.
/// * `shift` - The shift used in the Caesar cipher.
/// 
fn decrypt_caesar_cipher(text: &str, shift: u8) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                // Determine base character for case sensitivity
                let first_char = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                // Calculate and apply the shift
                let offset = c as u8 - first_char;
                let shifted = (offset + 26 - shift) % 26;
                (first_char + shifted) as char
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
    let shift: u8 = args[2].parse().expect("Shift must be a number");

    let decrypted_text = decrypt_caesar_cipher(encrypted_text, shift);
    println!("Decrypted Text: {}", decrypted_text);
}
