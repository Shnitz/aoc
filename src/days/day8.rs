use aoc::*;
use days::ChristmasDay;

pub struct Day8;

impl ChristmasDay for Day8 {
    fn solve(&self, _data: &str, _prob: ProblemPart) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day8_test1() {
        assert_eq!("", Day8.solve_a(""));
        assert_eq!("", Day8.solve_b(""));
    }
}
