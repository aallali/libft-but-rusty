
fn ft_get_sign(first_token: char) -> i8 {
    if first_token == '-' {
        -1
    } else if first_token == '+' {
        1
    } else {
        0
    }
}

/// Trims leading whitespace characters from the input string and returns a reference to the trimmed string.
///
/// # Arguments
///
/// * `nb_str` - A string slice (`&str`) to trim.
///
/// # Returns
///
/// A reference to the trimmed string slice. If the original string contains only whitespace characters, an empty string slice is returned.
///
/// # Examples
///
/// ```
/// let trimmed = ft_trim_begin_whitespaces("    Hello, world!");
/// assert_eq!(trimmed, "Hello, world!");
///
/// let all_whitespace = ft_trim_begin_whitespaces("     ");
/// assert_eq!(all_whitespace, "");
/// ```
fn ft_trim_begin_whitespaces(nb_str: &str) -> &str {
    // Create an iterator over the characters of the input string
    let mut chars = nb_str.chars();
    let mut index = 0;
    // Iterate over each character
    while let Some(c) = chars.next() {
        // If the character is not a whitespace, return the substring starting from that character

        if !c.is_whitespace() {
            return nb_str.get(index..).unwrap_or("");
        }

        index += 1;
    }

    // If all characters are whitespace, return an empty string
    ""
}

pub fn ft_atoi(mut nb_str: &str) -> i128 {
    nb_str = ft_trim_begin_whitespaces(nb_str);
    let mut sign = 1;
    let mut chars = nb_str.chars();
    let mut index: i128 = 0;
    let mut nb = 0;
    // Iterate over each character
    while let Some(c) = chars.next() {
        // If the character is not a whitespace, return the substring starting from that character

        if index == 0 && (c == '+' || c == '-') {
            sign = ft_get_sign(c);
        } else if c.is_digit(10) {
            nb = nb * 10 + (c as u8 - '0' as u8) as i128
        } else {
            break;
        }

        index += 1;
    }
    nb * i128::from(sign)
}
