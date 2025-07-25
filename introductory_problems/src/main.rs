use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::write;
use std::io::{self, BufRead, BufReader, StdinLock, BufWriter, StdoutLock}; // Import necessary modules
use std::io::Write;
use std::num::NonZeroU64;
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

fn getNthDig(num: u64, nth: u32, ndig:u32) -> u64{
    let mut n = num;
    if nth != 0{
        n = n/ 10_u64.pow(ndig - nth);
    }
    // remove last ndig -nth digits
    let last_digit = if n > 9 {
        n - ((n/10) * 10)} else{
        n
        };

    last_digit
}

fn testcase(reader:&mut BufReader<StdinLock<'static>> , writer: &mut BufWriter<StdoutLock<'static>>) {
    let mut n= read_string(reader);
    let n  = n.trim();
    let mut freq = vec![0;26];
    for c in n.chars() {
        freq[(c as u8 - b'A') as usize] += 1;
        
    }
    // println!("{:?}, {n}", freq);
    let mut last_used: Option<usize> = None;

    let total_chars_to_build = n.chars().count();
    let mut s = String::new();

    for i in 0..total_chars_to_build {
        let mut greatest_char_idx = 0;
        let mut max_count = 0;

        for (idx, &count) in freq.iter().enumerate() {
            if count > max_count {
                max_count = count;
                greatest_char_idx = idx;
            }
        }

        let mut smallest_available_char_idx= None;
        for (idx, &count) in freq.iter().enumerate() {
            if count > 0 && last_used != Some(idx) {
                smallest_available_char_idx = Some(idx);
                break;
            }
        }

        if max_count as u64 > ((total_chars_to_build - i) as u64 + 1) / 2 {
            println!("-1");
            return;
        }
        else if (total_chars_to_build - i) & 1 == 1 && max_count as u64 == ((total_chars_to_build - i) as u64 + 1) / 2 && last_used.unwrap_or(greatest_char_idx+1) != greatest_char_idx {
            let c = (greatest_char_idx as u8 + b'A' ) as char;
            s.push(c);
            freq[greatest_char_idx] -= 1;
            last_used = Some(greatest_char_idx);
        } else {
            let c =( smallest_available_char_idx.unwrap() as u8 + b'A' )as char;
            s.push(c);
            freq[smallest_available_char_idx.unwrap()] -= 1;
            last_used = Some(smallest_available_char_idx.unwrap());
        }

    }
    println!("{}", s);
    }
    

// [   |    ]
// cS >= mid
fn main() {
    let stdin = io::stdin(); // Get a handle to standard input
    let mut reader = io::BufReader::new(stdin.lock()); // Create a buffered reader
    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout.lock());
    // let t = read_int(&mut reader);
    // for _ in 0..t{
        testcase(&mut reader,&mut writer);
            // println!("NO");
        // };
    // }
    
}   