use std::collections::HashMap;
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
fn read_int_pair(reader: &mut BufReader<StdinLock<'static>>) -> (i32, i32) {
    let mut line = String::new();
    reader.read_line(&mut line)
    .expect("Failed to read N");
    let mut iter = line.split(" ").into_iter();
    let n: i32 = iter.next().unwrap().trim().parse().unwrap();
    let n1 = iter.next().unwrap().trim().parse().unwrap();
    (n, n1)
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
    let string = read_string(reader);
    let mut mp:HashMap<char, i32> = HashMap::new();
    let mut n = 0;
    for i in string.chars() {
        if i <= 'Z' && i >= 'A' {
            n +=1;
            *mp.entry(i).or_default() += 1;
        }
    }
    let mut out = vec!['A'; n];
    let mut odd_count = 0;
    for entry in mp.iter(){
        if entry.1 % 2 == 1 {
            odd_count += 1;
        }
    }
    if(odd_count > 1){
        writeln!(writer, "NO SOLUTION");
        return;
    }
    let mut left = 0;
    let mut right = n -1;
    for entry in mp.iter_mut(){
        let mut count  = *entry.1;
        if count & 1 == 1 {
            out[n/2] = *entry.0;
            count -= 1;
        }
        for _ in 0..(count / 2){
                out[left] = *entry.0;
                out[right] = *entry.0;
                left+=1;
                right-=1;
        }
    }
    writeln!(writer, "{}", out.iter().collect::<String>());
    
}

fn main() {
    let stdin = io::stdin(); // Get a handle to standard input
    let mut reader = io::BufReader::new(stdin.lock()); // Create a buffered reader
    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout.lock());
        testcase(&mut reader,&mut writer);
    
}   