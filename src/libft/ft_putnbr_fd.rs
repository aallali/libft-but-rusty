use super::{ft_itoa, ft_putstr_fd};
use std::fs::File;

pub fn ft_putnbr_fd(nbr: i128, file: &mut File) {
    ft_putstr_fd(ft_itoa(nbr), file);
}
