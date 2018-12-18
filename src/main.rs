extern crate aoc;
extern crate petgraph;

mod days;

use std::env;
use aoc::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    days::run(args[1].parse::<i8>().unwrap(), input(&args[1]));
}
