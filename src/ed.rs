use std::{
    fs::{File, OpenOptions},
    io::Write,
    io::{self, BufRead},
    path::Path,
};

pub struct Ed {
    pub buffer: Vec<String>,
}

impl Ed {
    pub fn new() -> Ed {
        Ed { buffer: Vec::new() }
    }

    pub fn print_all(&self) {
        for value in &self.buffer {
            println!("{}", value);
        }
    }

    pub fn append_lines(&mut self) {
        let stdin = io::stdin();
        let mut input = String::new();

        // loop to get appended text usings stdin
        loop {
            input.clear();
            stdin.read_line(&mut input).unwrap();
            let trimmed_input = input.trim();

            if trimmed_input == "." {
                break;
            }

            self.buffer.push(trimmed_input.to_string());
        }
    }

    // TODO: print range e.g. p 2,6
    pub fn print(&self, line_number: Option<usize>) {
        if let Some(line_num) = line_number {
            if line_num > 0 && line_num <= self.buffer.len() {
                println!("{}", self.buffer[line_num - 1]);
            } else {
                eprintln!("Invalid line number");
            }
        } else {
            for line in &self.buffer {
                println!("{}", line);
            }
        }
    }

    pub fn delete(&mut self, line_number: usize) {
        if line_number > 0 && line_number <= self.buffer.len() {
            self.buffer.remove(line_number - 1);
        } else {
            eprintln!("Invalid line number");
        }
    }

    pub fn write_to_file(&self, filename: &str) -> io::Result<()> {
        let path = Path::new(filename);
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;

        for line in &self.buffer {
            writeln!(file, "{}", line)?;
        }

        Ok(())
    }

    pub fn read_from_file(&mut self, filename: &str) -> io::Result<()> {
        let path = Path::new(filename);
        let file = File::open(path)?;

        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            self.buffer.push(line?);
        }
        self.print_all();

        Ok(())
    }
}
