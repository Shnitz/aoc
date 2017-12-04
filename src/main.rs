extern crate aoc;

mod days;

use std::env;
use aoc::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (a, b) = days::run(*&args[1].parse::<i32>().unwrap(), &input(&args[1]));
    println!("Part A: {}", a);
    println!("Part B: {}", b);
}
