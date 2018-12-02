use aoc::*;
use days::ChristmasDay;

pub struct Day9;

impl ChristmasDay for Day9 {
    fn solve(&self, _data: &str, _prob: ProblemPart) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day24_test1() {
        assert_eq!("", Day9.solve_a(""));
        assert_eq!("", Day9.solve_b(""));
    }
}