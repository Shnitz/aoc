use aoc::*;
use days::ChristmasDay;

pub struct Day13;

impl ChristmasDay for Day13 {
    fn solve(&self, _data: &str, _prob: ProblemPart) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day13_test1() {
        assert_eq!("", Day13.solve_a(""));
        assert_eq!("", Day13.solve_b(""));
    }
}
