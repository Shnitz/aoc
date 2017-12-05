use aoc::*;
use days::ChristmasDay;

pub struct Day17;

impl ChristmasDay for Day17 {
    fn solve(&self, _data: &str, _prob: ProblemPart) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day17_test1() {
        assert_eq!("", Day17.solve_a(""));
        assert_eq!("", Day17.solve_b(""));
    }
}