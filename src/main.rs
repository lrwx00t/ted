mod ed;
use ed::Ed;
use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    let mut ed = Ed::new();

    loop {
        let prompt = if ed.buffer.is_empty() {
            ": ".to_string()
        } else {
            format!("({}): ", ed.buffer.len())
        };
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let tokens: Vec<&str> = input.trim().split_whitespace().collect();

        match tokens.get(0) {
            Some(&"a") => {
                ed.append_lines();
            }
            Some(&"p") => {
                if tokens.len() > 1 {
                    if let Ok(line_number) = tokens[1].parse::<usize>() {
                        ed.print(Some(line_number));
                    } else {
                        eprintln!("Invalid line number");
                    }
                } else {
                    ed.print(None);
                }
            }
            Some(&",p") => ed.print_all(),
            Some(&"d") => {
                if let Some(line_number) = tokens.get(1) {
                    if let Ok(line_num) = line_number.parse::<usize>() {
                        ed.delete(line_num);
                    } else {
                        eprintln!("Invalid line number");
                    }
                } else {
                    eprintln!("Missing line number");
                }
            }
            Some(&"q") => break,
            Some(&"c") => ed.buffer.clear(),
            Some(&"w") => {
                if let Some(filename) = tokens.get(1) {
                    if let Err(err) = ed.write_to_file(filename) {
                        eprintln!("Error writing to file: {}", err);
                    }
                } else {
                    eprintln!("Missing filename");
                }
            }
            Some(&"r") => {
                if let Some(filename) = tokens.get(1) {
                    if let Err(err) = ed.read_from_file(filename) {
                        eprintln!("Error reading from file: {}", err);
                    }
                } else {
                    eprintln!("Missing filename");
                }
            }
            Some(&"h") | Some(&"help") => {
                println!("Available commands:");
                println!("a - Append lines to the buffer");
                println!("p [line_number] - Print line(s) from the buffer");
                println!("d [line_number] - Delete line from the buffer");
                println!("q - Quit the editor");
                println!("w <filename> - Write buffer contents to a file");
                println!("r <filename> - Read contents from a file into the buffer");
            }
            _ => {
                let invalid_command = tokens
                    .get(0)
                    .map(|&cmd| cmd.to_string())
                    .unwrap_or_default();
                eprintln!("Unknown command: {}", invalid_command);
            }
        }
    }
}
