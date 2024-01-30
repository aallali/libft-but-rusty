use std::str;
// use crate::libft::ft_putchar::ft_putchar; // or
use super::ft_putchar;


pub fn ft_putstr(string: &str){
    for c in string.chars() {
        ft_putchar(c)
    }
}
