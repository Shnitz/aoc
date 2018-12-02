use days::ChristmasDay;
use std::sync::mpsc;
use std::thread;
pub struct Day15;

impl ChristmasDay for Day15 {
    fn solve(&self, _data: &str, _prob: ProblemPart) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day24_test1() {
        assert_eq!("", Day24.solve_a(""));
        assert_eq!("", Day24.solve_b(""));
    }
}