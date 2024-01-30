use std::{fs::File, io::Write};


pub fn ft_putchar_fd(c: char, file: &mut File) {
    let bind = c.to_string();
	let temp = bind.as_bytes();
	file.write_all(temp).expect("Couldn't write to file");
}
