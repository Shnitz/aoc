use aoc::ProblemPart;
use days::ChristmasDay;

pub struct Day4;

impl ChristmasDay for Day4 {
    fn solve(&self, _data: &str, _prob: ProblemPart) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day24_test1() {
        assert_eq!("", Day4.solve_a(""));
        assert_eq!("", Day4.solve_b(""));
    }
}