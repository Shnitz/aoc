extern crate aoc;

mod day1;

use std::env;
use aoc::*;
use day1::day1;

fn main() {
    let args: Vec<String> = env::args().collect();
    let data: String = input(&args[1]);
    let (a1, a2) = match args[1].parse::<u32>().unwrap() {
        1 => (day1(&data, ProblemPart::A), day1(&data, ProblemPart::B)),
        _ => (String::from("NULL"), String::from("NULL")),
    };
    println!("Part A: {}", a1);
    println!("Part B: {}", a2);
}
