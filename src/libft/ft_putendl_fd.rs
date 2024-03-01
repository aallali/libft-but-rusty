use super::ft_putstr_fd;
use std::fs::File;

pub fn ft_putendl_fd(string: &str, file: &mut File) {
    ft_putstr_fd(&(string.to_owned() + "\n"), file);
}
