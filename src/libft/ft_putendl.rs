use std::str;
use super::ft_putstr;
use super::ft_putchar;

/// Print string given as argument followed by new line
///
/// # Arguments
///
/// * `string` - A string slice that holds the name of the person
///
/// # Examples
///
/// ```
/// ft_putendl("This sentence will be followed by a new line.")
/// ```
pub fn ft_putendl(string: &str) {
    ft_putstr(string);
    ft_putchar('\n');
}