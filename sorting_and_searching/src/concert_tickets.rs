/*

TLE - Solution

fn lower_bound(list: &Vec<i32>, elem: i32) -> usize{
    let mut left:i32 = 0;
    let mut right:i32 = list.len() as i32 - 1;
    let mut mid:i32 = 0;
    let mut ans = list.len() -1;
    while(left <= right){
        mid = left + ((right - left) >> 1);
        if list[mid as usize] > elem {
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
   let (ticket, customer) = read_int_pair(reader);
   let mut tickets = read_int_list(reader);
   let mut customers = read_int_list(reader);
   tickets.sort();
//    customers.sort();
//    tickets.reverse();
//    customers.reverse();
    let mut used = vec![false; ticket as usize];
//    let mut ans = vec![0; customers];
    // println!("{:?}", tickets);
   for c in customers{
    let mut ind = lower_bound(&tickets, c);
    let mut j:i32 = ind as i32;
    if tickets[ind] > c{
        j-=1;
    }
    // println!("{}", j);
    while j >= 0 && used[j as usize] {
        j-=1;
    }
    if j < 0{
        println!("-1");
        continue;
    }else{
        println!("{}",tickets[j as usize]);
        used[j as usize]= true;
    }
    }
    
}
    */

    use std::cmp::{max, min};
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::fmt::write;
use std::io::{self, BufRead, BufReader, StdinLock, BufWriter, StdoutLock}; // Import necessary modules
use std::io::Write;
use std::num::NonZeroU64;
use std::ops::Bound;
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
    let mut right:i32 = list.len() as i32 - 1;
    let mut mid:i32 = 0;
    let mut ans = list.len() -1;
    while(left <= right){
        mid = left + ((right - left) >> 1);
        if list[mid as usize] > elem {
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
   let (ticket, customer) = read_int_pair(reader);
   let mut tickets = read_int_list(reader);
   let mut customers = read_int_list(reader);
   tickets.sort();
//    customers.sort();
//    tickets.reverse();
//    customers.reverse();
    // let mut used = vec![false; ticket as usize];
    let mut mp:BTreeMap<i32, i32>  = BTreeMap::new();
    for t in &tickets{
        mp.entry(*t).and_modify(|count| *count += 1 ).or_insert(1);
    }
//    let mut ans = vec![0; customers];
    // println!("{:?}", tickets);
   for c in &customers{
    let mut iter = mp.range((Bound::Unbounded, Bound::Included(c)));
        if let Some((&chosen_price, &count)) = iter.next_back(){
            println!("{chosen_price}");
            if count == 1 {
                mp.remove(&chosen_price);
            } else {
                *mp.get_mut(&chosen_price).unwrap() -= 1;
            }
        } else {
            println!("-1");
        }
    }

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