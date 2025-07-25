/*

Brute Force - Wrong

fn lower_bound(list: &Vec<i32>, elem: i32) -> usize{
    let mut left:i32 = 0;
    let mut right:i32 = list.len() as i32;
    let mut mid:i32 = 0;
    let mut ans = list.len() -1;
    while(left <= right){
        mid = left + ((right - left) >> 1);
        if list[mid as usize] >= elem {
            ans = mid as usize;
            right = mid - 1;
        }
        else {
            left = mid + 1;
        }
    }
    // println!("ans {}", ans);
    ans as usize
}


fn testcase(reader:&mut BufReader<StdinLock<'static>> , writer: &mut BufWriter<StdoutLock<'static>>) {
   let (m, n, k) = read_int_triplet(reader);
   let mut list = read_int_list(reader);
   let mut apartment = read_int_list(reader);
   apartment.sort();
   let mut mp = HashMap::new();
   for i in &apartment{
    mp.entry(i).and_modify(|counter| *counter += 1).or_insert(1);
   }
//    println!("{:?}", mp);
   let mut ans = 0;
//    println!("{:?}", &list);
//    println!("{:?}", &apartment);
   for applicant in list{
    let mut ind = lower_bound(&apartment, applicant - k);
    while ind < apartment.len() - 1 && ((apartment[ind] <= applicant + k) && (apartment[ind] >= applicant-k) && mp.get(&apartment[ind]).unwrap() == &0){
        ind += 1;
    }
    if (apartment[ind] <= applicant + k) && (apartment[ind] >= applicant-k){
        // println!("{:?}", mp.get(&apartment[ind]));
        if mp.get(&apartment[ind]).unwrap() > &0{
            mp.entry(&apartment[ind]).and_modify(|counter| *counter -= 1);
            ans  += 1;
        }
    }
   }
   println!("{}", ans);
   
} */

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

fn lower_bound(list: &Vec<i32>, elem: i32) -> usize{
    let mut left:i32 = 0;
    let mut right:i32 = list.len() as i32;
    let mut mid:i32 = 0;
    let mut ans = list.len() -1;
    while(left <= right){
        mid = left + ((right - left) >> 1);
        if list[mid as usize] >= elem {
            ans = mid as usize;
            right = mid - 1;
        }
        else {
            left = mid + 1;
        }
    }
    // println!("ans {}", ans);
    ans as usize
}


fn testcase(reader:&mut BufReader<StdinLock<'static>> , writer: &mut BufWriter<StdoutLock<'static>>) {
   let (m, n, k) = read_int_triplet(reader);
   let mut applicants = read_int_list(reader);
   let mut apartments = read_int_list(reader);
   apartments.sort();
   applicants.sort();
//    applicants.reverse();
//    apartments.reverse();
// println!("{:?}", applicants);
//    println!("{:?}", apartments);
   

   let mut ans = 0;
   let mut j = 0;
   for i in 0 .. applicants.len(){
        while j < apartments.len() && apartments[j] < applicants[i] - k {
            j+=1;
        }
        if j == apartments.len() {
            break;
        }
        else{
            if(apartments[j] <= applicants[i] + k) {
                ans += 1;
                j+=1;
            }
        }
   }
   println!("{}", ans);
   
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