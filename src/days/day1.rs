use aoc::*;
use days::ChristmasDay;

pub struct Day1;

impl ChristmasDay for Day1 {
    fn solve(&self, data: &str, prob: ProblemPart) -> String {
        let chars: Vec<_> = data.chars().collect();
        let mut total = 0;
        let size = chars.len();
        let step = match prob {
            ProblemPart::A => 1,
            ProblemPart::B => size / 2
        };
        for idx in 0..size {
            if chars[idx] == chars[(idx + step) % size] {
                total += chars[idx].to_digit(10).unwrap();
            }
        }

        total.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day1_test1() {
        assert_eq!("3", Day1.solve_a("1122"));
        assert_eq!("4", Day1.solve_a("1111"));
        assert_eq!("0", Day1.solve_a("1234"));
        assert_eq!("9", Day1.solve_a("91212129"));
        assert_eq!("6", Day1.solve_b("1212"));
        assert_eq!("0", Day1.solve_b("1221"));
        assert_eq!("4", Day1.solve_b("123425"));
        assert_eq!("12", Day1.solve_b("123123"));
        assert_eq!("4", Day1.solve_b("12131415"));
    }
}
