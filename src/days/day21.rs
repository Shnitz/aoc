use aoc::*;
use days::ChristmasDay;

pub struct Day21;

impl ChristmasDay for Day21 {
    fn solve(&self, _data: &str, _prob: ProblemPart) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day21_test1() {
        assert_eq!("", Day21.solve_a(""));
        assert_eq!("", Day21.solve_b(""));
    }
}
