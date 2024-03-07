use std::fs::OpenOptions;
use std::i128::MAX as i128_MAX;
use std::i128::MIN as i128_MIN;
use std::i64::MAX as i64_MAX;
use std::i64::MIN as i64_MIN;
use std::io;

mod libft;

use libft::*;

fn main() -> io::Result<()> {
    ft_putendl("-------------------------------------------------: FT_PUTCHA/PUTSTR :----------------------------------------------");
    ft_putendl("-----------------------------------------------------------------------------------------------------------------");
    ft_putchar('A');
    ft_putchar('\n');

    ft_putstr("golden 3dida\n");
    ft_putendl("golden 3dida"); // same output as previous line
    ft_putendl("---------------------------------------------------: FT_..._FD :-------------------------------------------------");
    ft_putendl("-----------------------------------------------------------------------------------------------------------------");
    let mut file = match OpenOptions::new()
        .write(true)
        .create(true)
        .open("output.txt")
    {
        Ok(fd) => fd,
        Err(err) => {
            eprintln!("Error openning file: {}", err);
            return Err(err);
        }
    };
    ft_putendl_fd("Khurchuch", &mut file);

    ft_putchar_fd('_', &mut file);
    ft_putchar_fd('_', &mut file);

    file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("output.txt")?;

    ft_putstr_fd("golden 3dida", &mut file);
    ft_putchar_fd('_', &mut file);
    ft_putstr_fd("Silver 3dida", &mut file);
    ft_putendl("---------------------------------------------------: FT_PUTNBR :-------------------------------------------------");
    ft_putendl("-----------------------------------------------------------------------------------------------------------------");
    let nb: i32 = -999999999;
    ft_putnbr(i128::from(nb));
    ft_putchar('\n');

    ft_putnbr(1337);
    ft_putchar('\n');

    ft_putnbr(-1337);
    ft_putchar('\n');
    ft_putstr("");
    ft_putendl("----------------------------------------------------: FT_ITOA :--------------------------------------------------");
    ft_putendl("-----------------------------------------------------------------------------------------------------------------");
    ft_putendl(ft_itoa(i128::from(1334)));
    ft_putendl(ft_itoa(i128::from(-1334)));
    ft_putendl(ft_itoa(i128::from(-1)));
    ft_putendl(ft_itoa(i128::from(i128_MAX)));
    ft_putendl(ft_itoa(i128::from(i128_MIN)));
    ft_putendl(ft_itoa(i128::from(i64_MAX)));
    ft_putendl(ft_itoa(01));
    ft_putendl(ft_itoa(001234));
    ft_putendl(ft_itoa(-0));
    ft_putendl(ft_itoa(i128::from(i64_MIN)));
    ft_putendl("-------------------------------------------------: FT_PUTENDL_FD :-----------------------------------------------");
    ft_putendl("-----------------------------------------------------------------------------------------------------------------");
    ft_putendl_fd("Khurchuch", &mut file);
    ft_putendl_fd("Khurchuch", &mut file);
    ft_putnbr_fd(i128_MAX, &mut file);
    ft_putendl_fd("", &mut file);
    ft_putnbr_fd(i128_MIN, &mut file);
    ft_putendl("---------------------------------------------------: FT_ATOI :---------------------------------------------------");
    ft_putendl("-----------------------------------------------------------------------------------------------------------------");

    ft_putendl(&ft_atoi("-1234\n").to_string());
    ft_putendl(&ft_atoi("-42\034\n").to_string());

    ft_putendl(&ft_atoi("-1234 \n").to_string());
    ft_putendl(&ft_atoi("   - 1234 \n").to_string());
    ft_putendl(&ft_atoi("   1337 ").to_string());
    ft_putendl(&ft_atoi("   1a ").to_string());
    ft_putendl(&ft_atoi("   a1 ").to_string());
    ft_putendl(&ft_atoi("   123456789012345 ").to_string());
    ft_putendl(&ft_atoi("   -170141183460469231731687303715884105727 ").to_string());

    ft_putendl(&ft_atoi("   -170141183460469231731687303715884105728 ").to_string());

    Ok(())
}
