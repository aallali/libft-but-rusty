
/// # Description
/// convert an integer to ASCII 
///
/// # Arguments
///
/// * `i128` - A signed number of size i128
///
/// An i128 number is between:
/// - min :: -170_141_183_460_469_231_731_687_303_715_884_105_728i128
/// - max :: 170_141_183_460_469_231_731_687_303_715_884_105_727i128
/// 
/// # return
/// * `&'static str` - A given number as argument converted to String format
/// # Examples:
/// ```
/// ft_itoa(-1337); // return "-1337"
/// ft_itoa(42); // return "42"
/// ft_itoa(-1); // return "-1"
/// ft_itoa(-170141183460469231731687303715884105728) // return "-170141183460469231731687303715884105728"
/// ```
pub fn ft_itoa(mut nbr: i128) ->  &'static str{
    let mut nbr_str: String = String::from("");
    let mut is_negative: bool = false;


    if nbr < 0 {
        if nbr == -170141183460469231731687303715884105728 {
            return "-170141183460469231731687303715884105728";
        }
        is_negative = true;
        nbr = nbr * -1;
    }

    if nbr == 0 {
        return "0"
    }

    while nbr > 0 {
        nbr_str.insert(0, ((nbr % 10) as u8 + b'0') as char);
        nbr = nbr / 10;
    }

    if is_negative {
        nbr_str.insert(0, '-');
    }

    Box::leak(nbr_str.into_boxed_str())// equivalent to: return nbr_str;
}
