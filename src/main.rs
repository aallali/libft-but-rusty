
use std::fs::OpenOptions;
use std::i128::MAX as i128_MAX;
use std::i128::MIN as i128_MIN;
use std::i64::MAX as i64_MAX;
use std::i64::MIN as i64_MIN;
use std::io;

mod libft;


fn main() -> io::Result<()> {
    libft::ft_putchar('A');
    libft::ft_putchar('\n');

    libft::ft_putstr("golden 3dida\n");
    libft::ft_putendl("golden 3dida"); // same output as previous line

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
    libft::ft_putendl_fd("Khurchuch", &mut file);

    libft::ft_putchar_fd('_', &mut file);
    libft::ft_putchar_fd('_', &mut file);

    file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("output.txt")?;

    libft::ft_putstr_fd("golden 3dida", &mut file);
    libft::ft_putchar_fd('_', &mut file);
    libft::ft_putstr_fd("Silver 3dida", &mut file);

    let nb: i32 = -999999999;
    libft::ft_putnbr(i128::from(nb));
    libft::ft_putchar('\n');

    libft::ft_putnbr(1337);
    libft::ft_putchar('\n');

    libft::ft_putnbr(-1337);
    libft::ft_putchar('\n');
    libft::ft_putstr("");
    libft::ft_putendl(libft::ft_itoa(i128::from(1334)));
    libft::ft_putendl(libft::ft_itoa(i128::from(-1334)));
    libft::ft_putendl(libft::ft_itoa(i128::from(-1)));
    libft::ft_putendl(libft::ft_itoa(i128::from(i128_MAX)));
    libft::ft_putendl(libft::ft_itoa(i128::from(i128_MIN)));
    libft::ft_putendl(libft::ft_itoa(i128::from(i64_MAX)));
    libft::ft_putendl(libft::ft_itoa(01));
    libft::ft_putendl(libft::ft_itoa(001234));
    libft::ft_putendl(libft::ft_itoa(-0));
    libft::ft_putendl(libft::ft_itoa(i128::from(i64_MIN)));

    libft::ft_putendl_fd("Khurchuch", &mut file);
    libft::ft_putendl_fd("Khurchuch", &mut file);
    libft::ft_putnbr_fd(i128_MAX, &mut file);
    libft::ft_putendl_fd("", &mut file);

    libft::ft_putnbr_fd(i128_MIN, &mut file);

    
    Ok(())
}
