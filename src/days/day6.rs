use aoc::*;
use days::ChristmasDay;

pub struct Day6;

impl ChristmasDay for Day6 {
    fn solve(&self, _data: &str, _prob: ProblemPart) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day6_test1() {
        assert_eq!("", Day6.solve_a(""));
        assert_eq!("", Day6.solve_b(""));
    }
}
