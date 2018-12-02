use aoc::*;
use days::ChristmasDay;
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::{ Sender, Receiver };
use std::thread;
use std::time::Duration;

pub struct Day18;

impl ChristmasDay for Day18 {
    fn solve(&self, _data: &str, _prob: ProblemPart) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day24_test1() {
        assert_eq!("", Day24.solve_a(""));
        assert_eq!("", Day24.solve_b(""));
    }
}