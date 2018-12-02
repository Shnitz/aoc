use aoc::*;
use days::ChristmasDay;

pub struct Day10;

impl ChristmasDay for Day10 {
    fn solve(&self, _data: &str, _prob: ProblemPart) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day24_test1() {
        assert_eq!("", Day10.solve_a(""));
        assert_eq!("", Day10.solve_b(""));
    }
}