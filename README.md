<img src="https://user-images.githubusercontent.com/8974888/231858967-7c37bf1e-335b-4f5a-9760-da97be9f54bb.png" width="200" />

# Libft But Rusty

### A Reimplementation of 42's Common Core _Libft_ in Rust, to learn Rust

re-implementing a set of Libc functions + some others to rust

I tried to follow the same limitations that the 'libft' subject gives us, such as limiting memory allocation in fuctions that don't use it in the libc. However I folowed the originial limitations of the project and their c implementations loosly.


### functions:
- Note: _`(click on func name to view source code`)_
- Note: (`🧪`): _means unit tests are written for the function_
- Note: _`(click on 🧪 to view unit tests`)_
- Note: (`⏳`): _means function under progress_

### part 1 (output funcs):
1. ##### output to stdout:
- [x] [`ft_putchar`](./src/libft/ft_putchar.rs)
- [x] [`ft_putstr`](./src/libft/ft_putstr.rs)
- [x] [`ft_putendl`](./src/libft/ft_putendl.rs)
- [x] [`ft_putnbr`](./src/libft/ft_putnbr.rs)

1. ##### output to file:
- [x] [`ft_putchar_fd`](./src/libft/ft_putchar_fd.rs) [🧪](./src/tests/ft_putchar_fd_test.rs)
- [x] [`ft_putstr_fd`](./src/libft/ft_putstr_fd.rs) [🧪](./src/tests/ft_putstr_fd_test.rs)
- [x] [`ft_putendl_fd`](./src/libft/ft_putendl_fd.rs)
- [x] [`ft_putnbr_fd`](./src/libft/ft_putnbr_fd.rs) [🧪](./src/tests/ft_putnbr_fd_test.rs)

### part 2 (converters):
- [x] [`ft_itoa`](./src/libft/ft_itoa.rs) [🧪](./src/tests/ft_itoa_test.rs)
- [ ] ⏳ [`ft_atoi`]
- [ ] `ft_toupper`
- [ ] `ft_tolower`

### part 3 (validators):
- [ ] `ft_isalpha`
- [ ] `ft_isdigit`
- [ ] `ft_isalnum`
- [ ] `ft_isascii`
- [ ] `ft_isprint`

### part 4 (string handlers):
- [ ] `ft_strcmp`
- [ ] `ft_strncmp`
- [ ] `ft_strequ`
- [ ] `ft_strnequ`
- [ ] `ft_strsub`
- [ ] `ft_strjoin`
- [ ] `ft_strtrim`
- [ ] `ft_strsplit`
- [ ] `ft_strlen`
- [ ] `ft_strdup`
- [ ] `ft_strcpy`
- [ ] `ft_strncpy`
- [ ] `ft_strcat`
- [ ] `ft_strncat`
- [ ] `ft_strlcat`
- [ ] `ft_strchr`
- [ ] `ft_strrchr`
- [ ] `ft_strstr`
- [ ] `ft_strnstr`

### part 5 (memory):
- [ ] `ft_memset`
- [ ] `ft_bzero`
- [ ] `ft_memcpy`
- [ ] `ft_memccpy`
- [ ] `ft_mesmmove`
- [ ] `ft_memchr`
- [ ] `ft_memcmp`


### part 6 (linked lists):
- [ ] `ft_lstnew`
- [ ] `ft_lstdelone`
- [ ] `ft_lstdel`
- [ ] `ft_lstadd`
- [ ] `ft_lstiter`
- [ ] `ft_lstmap`

