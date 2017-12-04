extern crate aoc;

mod day1;
mod day2;
mod day3;
mod day4;

use std::env;
use aoc::*;
use day1::day1;
use day2::day2;
use day3::day3;
use day4::day4;

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
