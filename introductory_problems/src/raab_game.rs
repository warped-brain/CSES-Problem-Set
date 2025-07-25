use std::cmp::{max, min};
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
fn read_int_triplet(reader: &mut BufReader<StdinLock<'static>>) -> (i32, i32, i32) {
    let mut line = String::new();
    reader.read_line(&mut line)
    .expect("Failed to read N");
    let mut iter = line.split(" ").into_iter();
    let n: i32 = iter.next().unwrap().trim().parse().unwrap();
    let n1 = iter.next().unwrap().trim().parse().unwrap();
    let n2 = iter.next().unwrap().trim().parse().unwrap();
    (n, n1, n2)
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
fn write_string_list(writer: &mut BufWriter<StdoutLock<'static>>, list: Vec<String>) {
    for word in list{
        let _ = writeln!(writer, "{}", word);
    }
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

fn helper(row: usize,  board:&mut Vec<String>, ans: &mut u64, placedAbove: &mut Vec<bool>, diagLR: &mut Vec<bool>, diagRL: &mut Vec<bool>){
    if row == 8 {
        *ans += 1;
        return;
    }
    for col in 0..8_usize{
        let line = board[row].trim();
        let cell_char = line.chars().nth(col).unwrap();

        let diag_lr_idx = (row as isize - col as isize + 7) as usize;

        let diag_rl_idx = row + col;

        if cell_char == '.' && !placedAbove[col] && !diagLR[diag_lr_idx] && !diagRL[diag_rl_idx] {
            placedAbove[col] = true;
            diagLR[diag_lr_idx] = true;
            diagRL[diag_rl_idx] = true;

            helper(row + 1, board, ans, placedAbove, diagLR, diagRL);

            // Backtrack
            placedAbove[col] = false;
            diagLR[diag_lr_idx] = false;
            diagRL[diag_rl_idx] = false;
        }
    }
}

fn testcase(reader:&mut BufReader<StdinLock<'static>> , writer: &mut BufWriter<StdoutLock<'static>>) -> bool {
    let(mut n, a, b) = read_int_triplet(reader);
    let mut a_plays = Vec::new();
    let mut b_plays = Vec::new();
    let ties = n - a -b;
    n -= ties;
    if ties < 0 {
        return false;
    }
    if a+b == 1 {
        return false;
    }
    else if n != 0 && (a == n || b== n){
        return false;
    }
    else {
        println!("YES");
        
        for i in n+1..=n+ties{
            a_plays.push(i);
            b_plays.push(i);
        }
        for i in(1..=n).rev(){
            a_plays.push(i);
            // println!("n = {n}, val = {}", n-i+1);

            if(n - i < a) {
                b_plays.push(a - (n-i));
            }
            else{
                b_plays.push(i+a);
            }
        }
        for i in a_plays{
            print!("{i} ");
        }
        println!("");
        for i in b_plays{
            print!("{i} ");
        }
        println!("");
        return true;
    }

    }
    

// [   |    ]
// cS >= mid

fn main() {
    let stdin = io::stdin(); // Get a handle to standard input
    let mut reader = io::BufReader::new(stdin.lock()); // Create a buffered reader
    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout.lock());
    let t = read_int(&mut reader);
    for _ in 0..t{
        if(!testcase(&mut reader,&mut writer)) {
            println!("NO");
        };
    }
    
}   