use std::fmt::write;
use std::io::{self, BufRead, BufReader, StdinLock, BufWriter, StdoutLock}; // Import necessary modules
use std::io::Write;
fn read_int(reader: &mut BufReader<StdinLock<'static>>) -> i32 {
    let mut line = String::new();
    reader.read_line(&mut line)
    .expect("Failed to read N");

    let n: i32 = line.trim().parse().unwrap();
    n
}
fn read_string(reader: &mut BufReader<StdinLock<'static>>) -> String {
    let mut line = String::new();
    reader.read_line(&mut line)
    .expect("Failed to read N");
    line
}
fn write_int(writer: &mut BufWriter<StdoutLock<'static>>, n : i32) {
    let _ = write!(writer, "{n}");
}
fn write_string(writer: &mut BufWriter<StdoutLock<'static>>, word: String) {
    let _ = write!(writer, "{}", word);
}
fn write_u64(writer: &mut BufWriter<StdoutLock<'static>>, n : u64) {
    let _ = write!(writer, "{n}");
}
fn read_int_list(reader: &mut BufReader<StdinLock<'static>>) -> Vec<i32>{
    
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let list = line.split_ascii_whitespace().map(|x| x.parse().unwrap()).collect();
    list

}

fn read_uint64_list(reader: &mut BufReader<StdinLock<'static>>) -> Vec<u64>{
    
    let mut line = String::new();   
    reader.read_line(&mut line).unwrap();
    let list = line.split_ascii_whitespace().map(|x| x.parse().unwrap()).collect();
    list

}
fn testcase(reader:&mut BufReader<StdinLock<'static>> , writer: &mut BufWriter<StdoutLock<'static>>) {
    let mut n = read_int(reader);
    let mut A: Vec<i32> = Vec::new();
    let mut B: Vec<i32> = Vec::new();
    let mut sumA:i64 = 0;
    let mut sumB:i64 = 0;
    while n > 0 {
        if(sumA >= sumB){
            sumB += n as i64;
            B.push(n);
        }
        else{
            sumA += n as i64;
            A.push(n);
        }
        n-=1;
    }
    if sumA != sumB {
        writeln!(writer, "NO");
        return;
    }
    else {
        writeln!(writer, "YES");
    }
    writeln!(writer, "{}", A.len());
    for i in A{
        write!(writer, "{i} ");
    }
    writeln!(writer, "\n{}", B.len());
    for i in B{
        write!(writer, "{i} ");
    }
}
fn main() {
    let stdin = io::stdin(); // Get a handle to standard input
    let mut reader = io::BufReader::new(stdin.lock()); // Create a buffered reader
    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout.lock());
    testcase(&mut reader,&mut writer);
    
}   