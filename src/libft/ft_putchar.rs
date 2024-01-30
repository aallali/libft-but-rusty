 
use std::io::{self, Write};

// Writes single character to stdout.
pub fn ft_putchar(c : char) {
	let bind: String = String::from(c); // == c.to_string();
	let temp: &[u8] = bind.as_bytes();
    // print ASCII code for this character (eg: A==65)
    // println!("{:?}", temp);
    io::stdout().write_all(temp).expect("Failed to write to stdout");
}