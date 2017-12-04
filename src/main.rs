extern crate aoc;

mod days;

use std::env;
use aoc::*;
use days::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let data: String = input(&args[1]);
    let (a, b) = match args[1].parse::<u32>().unwrap() {
        1 => (day1(&data, ProblemPart::A), day1(&data, ProblemPart::B)),
        2 => (day2(&data, ProblemPart::A), day2(&data, ProblemPart::B)),
        3 => (day3(&data, ProblemPart::A), day3(&data, ProblemPart::B)),
        4 => (day4(&data, ProblemPart::A), day4(&data, ProblemPart::B)),
        _ => (String::from("NULL"), String::from("NULL")),
    };
    println!("Part A: {}", a);
    println!("Part B: {}", b);
}
