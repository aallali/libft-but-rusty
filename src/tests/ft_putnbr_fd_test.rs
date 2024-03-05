#[path = "../libft/mod.rs"]
mod libft;
use libft::ft_putnbr_fd;

use std::fs;
use std::i128;
use std::i8;
use std::io::{self, Read};


use crate::ft_putnbr_fd_test::libft::ft_putendl;

fn use_ft_putnbr_fd(file_name: &str, nb_test: i128) -> io::Result<String> {
    let mut file: fs::File = match fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_name)
    {
        Ok(fd) => fd,
        Err(err) => {
            println!("Error openning file: {}", err);
            return Err(err);
        }
    };

    ft_putnbr_fd(nb_test, &mut file);
    let mut file: fs::File = fs::File::open(file_name)?;
    let mut contents = String::new();

    // Read the first character of the file
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn ft_putnbr_fd_mock(input_test: i128, expected_content: &str, rand_suffix: i8) {
    let file_name: String = format!("./src/tests/output_test_{rand_suffix}.txt");

    // Call the function to read the file content
    match use_ft_putnbr_fd(file_name.as_str(), input_test) {
        Ok(content) => {
            // Assert that the content of the file matches the expected content
            if !content.starts_with(expected_content) {
                ft_putendl("-------");

                ft_putendl(&content);
                ft_putendl(&expected_content);
                ft_putendl("----------");

            }
            assert_eq!(content.starts_with(expected_content), true);
        }
        Err(e) => {
            // If there's an error reading the file, fail the test
            panic!("Error reading file----: {:?}", e);
        }
    }
    let _ = fs::remove_file(file_name);
}
#[cfg(test)]
mod tests {
    use super::ft_putnbr_fd_mock;

    #[test]
    fn test_putnbr_fd_range_of_max() {
        ft_putnbr_fd_mock(i128::MAX, &i128::MAX.to_string(), 0);
        ft_putnbr_fd_mock(i128::MIN, &i128::MIN.to_string(),0);

        ft_putnbr_fd_mock(i128::from(i64::MAX), &i64::MAX.to_string(),0);
        ft_putnbr_fd_mock(i128::from(i64::MIN), &i64::MIN.to_string(),0);

        ft_putnbr_fd_mock(i128::from(i32::MAX), &i32::MAX.to_string(),0);
        ft_putnbr_fd_mock(i128::from(i32::MIN), &i32::MIN.to_string(),0);

        ft_putnbr_fd_mock(i128::from(i16::MAX), &i16::MAX.to_string(),0);
        ft_putnbr_fd_mock(i128::from(i16::MIN), &i16::MIN.to_string(),0);

        ft_putnbr_fd_mock(i128::from(i8::MAX), &i8::MAX.to_string(),0);
        ft_putnbr_fd_mock(i128::from(i8::MIN), &i8::MIN.to_string(),0);
    }

    #[test]
    fn test_putnbr_fd_random() {
        ft_putnbr_fd_mock(42, "42",33);
        ft_putnbr_fd_mock(1337, "1337", 33);

        // ft_putnbr_fd_mock(01, "1");
    }
}
