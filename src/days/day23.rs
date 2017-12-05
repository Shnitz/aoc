use aoc::*;
use days::ChristmasDay;

pub struct Day23;

impl ChristmasDay for Day23 {
    fn solve(&self, _data: &str, _prob: ProblemPart) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day23_test1() {
        assert_eq!("", Day23.solve_a(""));
        assert_eq!("", Day23.solve_b(""));
    }
}
