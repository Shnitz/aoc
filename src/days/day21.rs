use aoc::*;
use days::ChristmasDay;
// use na::{Matrix, Matrix2, Matrix3, Matrix4, MatrixVec, DMatrix};

pub struct Day21;

impl ChristmasDay for Day21 {
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