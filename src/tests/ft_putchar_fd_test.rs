#[path = "../libft/ft_putchar_fd.rs"] mod ft_putchar_fd;
use ft_putchar_fd::ft_putchar_fd;
use std::fs;
use std::io::{self, Read};

// In the line let mut buffer = [0; 1];, [0; 1] is an array initialization syntax in Rust.

// The 0 inside the brackets represents the initial value of each element in the array. In this case, it means that each element of the array will be initialized with the value 0.

// The 1 following the semicolon specifies the length of the array. In this case, it means that the array will have a length of 1, so it will contain one element.

// 1. `let mut buffer = [0; 1];`: This line declares a mutable variable `buffer` of type array `[i32; 1]`. The array contains one element initialized with `0`. This array will be used to store the bytes read from the file.

// 2. `file.read_exact(&mut buffer)?;`: This line reads exactly enough bytes from the file to fill the buffer. The `read_exact` method attempts to read bytes from the file and store them into the provided buffer until the buffer is filled or an error occurs. The `&mut buffer` is a mutable reference to the buffer, allowing the method to modify the buffer directly. The `?` operator is used here to handle any potential errors that may occur during the reading process. If an error occurs, the function returns early with the error, otherwise, it continues.

// After this line executes, the `buffer` array will contain the bytes read from the file, specifically, in this case, it will hold the first byte of the file content.
fn use_ft_putchar_fd(file_name: &str, char_test: char) -> io::Result<char> {
    let mut file: fs::File = match fs::OpenOptions::new().write(true).create(true).open(file_name) {
        Ok(fd) => fd,
        Err(err) => {
            println!("Error openning file: {}", err);
            return Err(err);
        }
    };

    ft_putchar_fd(char_test, &mut file);
    let mut file: fs::File = fs::File::open(file_name)?;
    // Read the first character of the file
    let mut buffer: [u8; 1] = [0; 1];
    file.read_exact(&mut buffer)?;

    // file.read_to_string(&mut contents)?;

    // Convert the first character to a char
    let first_char = buffer[0] as char;
    Ok(first_char)
}

#[test]
fn test_putchar_fd() {
    let input_test: char = '+';
    let expected_content: char = '+'; // Replace with the expected content of the file
    let file_name: &str = "./src/tests/output_test.txt";
    // Call the function to read the file content
    match use_ft_putchar_fd(file_name, input_test) {
        Ok(first_char) => {
            // Assert that the content of the file matches the expected content
            assert_eq!(first_char, expected_content);
        }
        Err(e) => {
            // If there's an error reading the file, fail the test
            panic!("Error reading file----: {:?}", e);
        }
    }
}
