use aoc::*;
use days::ChristmasDay;

pub struct Day22;

impl ChristmasDay for Day22 {
    fn solve(&self, _data: &str, _prob: ProblemPart) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day24_test1() {
        assert_eq!("", Day22.solve_a(""));
        assert_eq!("", Day22.solve_b(""));
    }
}