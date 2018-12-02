use aoc::*;
use days::ChristmasDay;
use days::day10::Day10;
use std::collections::VecDeque;

pub struct Day14;

impl ChristmasDay for Day14 {
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