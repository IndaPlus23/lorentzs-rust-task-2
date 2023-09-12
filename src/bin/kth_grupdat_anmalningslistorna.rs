#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(warnings)]

use std::collections::{HashMap, HashSet};
use std::io;
use std::io::BufRead;
use std::ops::Deref;

fn main() {
    let input = io::stdin();
    let mut n_line = String::new();
    input.lock().read_line(&mut n_line).unwrap();
    let n: i32 = n_line.trim().parse::<i32>().unwrap();
    let mut first_names: Vec<String> = Vec::new();
    let mut last_names: Vec<String> = Vec::new();
    for i in 0..n {
        let mut line = String::new();
        input.lock().read_line(&mut line).unwrap();
        first_names.push(String::from(line.trim().to_lowercase()));
    }
    for i in 0..n {
        let mut line = String::new();
        input.lock().read_line(&mut line).unwrap();
        last_names.push(String::from(line.trim().to_lowercase()));
    }
    let mut result_set: HashSet<String> = HashSet::new();
    for i in 0..n {
        let name = format!("{} {}",first_names[i as usize].clone(),last_names[i as usize].clone());
        result_set.insert(name);
    }
    println!("{}", result_set.len());
}