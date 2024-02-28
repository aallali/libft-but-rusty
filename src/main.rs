use std::fs::OpenOptions;
use std::io;
mod libft;

fn main() -> io::Result<()> {
    libft::ft_putchar('A');
    libft::ft_putchar('\n');

    libft::ft_putstr("golden 3dida\n");
    libft::ft_putendl("golden 3dida"); // same output as previous line

    let mut file = match OpenOptions::new().write(true).create(true).open("output.txt") {
        Ok(fd) => fd,
        Err(err) => {
            eprintln!("Error openning file: {}", err);
            return Err(err);
        }
    };

    libft::ft_putchar_fd('_', &mut file);
    libft::ft_putchar_fd('_', &mut file);

    file = OpenOptions::new().write(true).open("output.txt")?;

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
    Ok(())
}
