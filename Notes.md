


### imports:
```rust
use std::io::{self, Write};
or
use std::io;
use std::io::Write;
or
use std::io::Write;
use std::io::stdout;
```
all valid to import IO+Write, except the last one it export stdout directly, so we dont need to use `io::stdout()`
```rust
io::stdout().write_all(temp).expect("Failed to write to stdout");
```

## write vs write_all:
In Rust, the `write_all` method and the `write` method are both part of the `Write` trait, but they have different behaviors:

1. **`write_all` method:**
   - **Signature:** `fn write_all(&mut self, buf: &[u8]) -> Result<()>`
   - **Behavior:** Writes the entire buffer `buf` to the writer, ensuring that all bytes are written.
   - **Return Value:** Returns a `Result` indicating whether the write operation was successful (`Ok(())`) or if an error occurred (`Err`).

2. **`write` method:**
   - **Signature:** `fn write(&mut self, buf: &[u8]) -> Result<usize>`
   - **Behavior:** Writes as many bytes as possible from the buffer `buf` to the writer, returning the number of bytes actually written.
   - **Return Value:** Returns a `Result` with the number of bytes written (`Ok(usize)`) or an error (`Err`).

- `write_all` ensures that the entire buffer is written and may block until that is achieved.
- `write` writes as many bytes as possible and returns the number of bytes written, allowing partial writes.

When using `write_all`, you typically expect either all bytes to be written successfully or none at all. On the other hand, when using `write`, you may need to check the return value to see how many bytes were actually written and potentially handle partial writes.