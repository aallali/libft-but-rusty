use super::ft_putchar;

/// # Description
/// print a number to stdout
///
/// # Arguments
///
/// * `i128` - A signed numberof size i128
///
/// An i128 number is between:
/// - min :: -170_141_183_460_469_231_731_687_303_715_884_105_728i128
/// - max :: 170_141_183_460_469_231_731_687_303_715_884_105_727i128
/// 
/// # Examples:
/// ```
/// ft_putnbr(-1337); // output "-1337%" // '%' mean end of output
/// ft_putnbr(42); // output "42%"
/// ft_putnbr(-1); // output "-1%"
/// ```
pub fn ft_putnbr(nb: i128) {
    let unb:i128;

    if nb < 0 {
        ft_putchar('-');
        unb = nb * -1;
    } else {
        unb = nb
    }
    if unb > 9 {
        ft_putnbr(unb / 10);
    }

    ft_putchar(((unb % 10) as u8 + b'0') as char);


    // in parallel world where developers are less autistic you can use:
    // pub fn ft_putnbr(n: i32) -> io::Result<()> {
    //     let s: String = n.to_string();
    //     ft_putstr(&(s.to_owned()))
    // }
}
