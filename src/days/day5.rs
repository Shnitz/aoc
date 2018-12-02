use aoc::*;
use days::ChristmasDay;

pub struct Day5;

impl ChristmasDay for Day5 {
    fn solve(&self, _data: &str, _prob: ProblemPart) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day24_test1() {
        assert_eq!("", Day5.solve_a(""));
        assert_eq!("", Day5.solve_b(""));
    }
}