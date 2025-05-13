use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line.trim() == "ping" {
            writeln!(stdout, "pong").unwrap();
            stdout.flush().unwrap();
        }
    }
}
