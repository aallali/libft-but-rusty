use std::io;
use std::fs::OpenOptions;

mod libft;


fn main() -> io::Result<()> {
    libft::ft_putchar('A');
    libft::ft_putchar('\n');

    libft::ft_putstr("Abdellah Allali\n");
    libft::ft_putchar('-');

    let mut file = match OpenOptions::new().write(true).open("output.txt") {
        Ok(fd) => fd,
        Err(err) => {
            eprintln!("Error openning file: {}", err);
            return Err(err);
        }
    };

    libft::ft_putchar_fd('_', &mut file);

    Ok(())
}
