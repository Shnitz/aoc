use aoc::*;
use days::ChristmasDay;

pub struct Day16;

impl ChristmasDay for Day16 {
    fn solve(&self, _data: &str, _prob: ProblemPart) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day16_test1() {
        assert_eq!("", Day16.solve_a(""));
        assert_eq!("", Day16.solve_b(""));
    }
}
