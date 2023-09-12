#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(warnings)]

use std::io;
use std::io::prelude::*;
use std::collections::BTreeSet;

fn main() {
    let input = io::stdin();
    let mut line1 = String::new();
    input.lock().read_line(&mut line1).unwrap();
    let n = line1.trim().parse::<i32>().unwrap();
    let searches = if n % 2 == 0 { n/2 } else { (n+1)/2 };
    let mut line2 = String::new();
    input.lock().read_line(&mut line2).unwrap();
    let mut nums: Vec<i32> = Vec::new();
    line2.trim()
        .split(" ")
        .map(|_token| _token.parse::<i32>())
        .for_each(|_token| {
            if _token.is_ok() {
                let i1 = _token.unwrap();
                nums.push(i1);
            }
        });
    nums.sort_by(|_num1, _num2| _num1.cmp(_num2));
    let sum = nums.iter()
        .skip((n - searches) as usize)
        .map(|_x| *_x)
        .reduce(|_num1, _num2| _num2 + _num1)
        .unwrap();
    println!("{}", sum);
}
