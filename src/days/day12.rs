extern crate petgraph;

use aoc::*;
use days::ChristmasDay;
use std::collections::HashMap;

use self::petgraph::visit::{Bfs, Walker};

pub struct Day12;

impl ChristmasDay for Day12 {
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