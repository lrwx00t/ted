# ted
Tiny Ed implemented in Rust

## Usage Example

```bash
❯ cargo run
   Compiling ted v0.1.0 (ted)
    Finished dev [unoptimized + debuginfo] target(s) in 0.52s
     Running `target/debug/ted`
: h
Available commands:
a - Append lines to the buffer
p [line_number] - Print line(s) from the buffer
d [line_number] - Delete line from the buffer
q - Quit the editor
w <filename> - Write buffer contents to a file
r <filename> - Read contents from a file into the buffer
```
