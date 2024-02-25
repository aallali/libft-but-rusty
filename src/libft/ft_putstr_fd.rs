use std::{fs::File, io::Write};


pub fn ft_putstr_fd(string: &str, file: &mut File) {
	
	let temp: &[u8] = string.as_bytes();
	file.write_all(temp).expect("Couldn't write to file");
}
