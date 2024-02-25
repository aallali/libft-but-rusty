#[path = "../libft/ft_putstr_fd.rs"] mod ft_putstr_fd;
use ft_putstr_fd::ft_putstr_fd;
use std::fs;
use std::io::{self, Read};

fn use_ft_putstr_fd(file_name: &str, str_test: &str) -> io::Result<String> {
    let mut file: fs::File = match fs::OpenOptions::new().write(true).create(true).open(file_name) {
        Ok(fd) => fd,
        Err(err) => {
            println!("Error openning file: {}", err);
            return Err(err);
        }
    };

    ft_putstr_fd(str_test, &mut file);
    let mut file = fs::File::open(file_name)?;
    // Read the first character of the file
    // let mut buffer: Vec<u8> = vec![0; str_test.len()];
    // file.read_exact(&mut buffer)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Convert the first character to a char
    // let string_from_buffer = String::from_utf8(buffer).expect("Conversion from Vec<u8> to String failed");

    Ok(contents)
}

#[test]
fn test_putstr_fd() {
    let mut input_test: &str = "1234";
    let mut expected_content:  &str = "1234"; // Replace with the expected content of the file
    let file_name: &str = "./src/tests/output_test2.txt";
    // Call the function to read the file content
    match use_ft_putstr_fd(file_name, input_test) {
        Ok(first_char) => {
            // Assert that the content of the file matches the expected content
            assert_eq!(first_char, expected_content);
        }
        Err(e) => {
            // If there's an error reading the file, fail the test
            panic!("Error reading file: {:?}", e);
        }
    }
    input_test = "X";
    expected_content = "X234"; // Replace with the expected content of the file

    // Call the function to read the file content
    match use_ft_putstr_fd(file_name, input_test) {
        Ok(first_char) => {
            // Assert that the content of the file matches the expected content
            assert_eq!(first_char, expected_content);
        }
        Err(e) => {
            // If there's an error reading the file, fail the test
            panic!("Error reading file: {:?}", e);
        }
    }
    let _ = fs::remove_file(file_name);
}
