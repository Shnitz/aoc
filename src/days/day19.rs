use aoc::*;
use days::ChristmasDay;

pub struct Day19;

impl ChristmasDay for Day19 {
    fn solve(&self, _data: &str, _prob: ProblemPart) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day19_test1() {
        assert_eq!("", Day19.solve_a(""));
        assert_eq!("", Day19.solve_b(""));
    }
}
