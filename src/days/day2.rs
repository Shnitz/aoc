use aoc::*;
use days::ChristmasDay;

pub struct Day2;

impl ChristmasDay for Day2 {
    fn solve(&self, _data: &str, _prob: ProblemPart) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day24_test1() {
        assert_eq!("", Day2.solve_a(""));
        assert_eq!("", Day2.solve_b(""));
    }
}