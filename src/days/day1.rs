use aoc::*;
use days::ChristmasDay;

pub struct Day1;

impl ChristmasDay for Day1 {
    fn solve(&self, _data: &str, _prob: ProblemPart) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day24_test1() {
        assert_eq!("", Day1.solve_a(""));
        assert_eq!("", Day1.solve_b(""));
    }
}