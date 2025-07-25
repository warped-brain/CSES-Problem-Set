use std::io::{self, BufRead}; // Import necessary modules
use std::io::Write;
fn main() {
    let stdin = io::stdin(); // Get a handle to standard input
    let mut reader = io::BufReader::new(stdin.lock()); // Create a buffered reader
    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout.lock());
    let mut line = String::new();
    reader.read_line(&mut line)
        .expect("Failed to read N");

    let mut n: i32 = line.trim().parse()
        .expect("Failed to parse integer from line"); // Error message for parsing failure
    write!(writer, "{n}");
    while n != 1 {
        if n & 1 == 1 {
            n = (n *3) + 1;
            write!(writer, " {n}");
        }
        else {
            n = n/2;
            write!(writer, " {n}");
        }
    }
    write!(writer, "\n");
}