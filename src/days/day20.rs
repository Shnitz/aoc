use aoc::*;
use days::ChristmasDay;

pub struct Day20;

impl ChristmasDay for Day20 {
    fn solve(&self, _data: &str, _prob: ProblemPart) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day20_test1() {
        assert_eq!("", Day20.solve_a(""));
        assert_eq!("", Day20.solve_b(""));
    }
}
