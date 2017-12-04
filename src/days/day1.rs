use aoc::*;
use days::ChristmasDay;

pub struct Day1;

impl ChristmasDay for Day1 {
    fn solve(&self, data: &String, prob: ProblemPart) -> String {
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
    fn test1() {
        assert_eq!("3", Day1.solve(&String::from("1122"), ProblemPart::A));
        assert_eq!("4", Day1.solve(&String::from("1111"), ProblemPart::A));
        assert_eq!("0", Day1.solve(&String::from("1234"), ProblemPart::A));
        assert_eq!("9", Day1.solve(&String::from("91212129"), ProblemPart::A));
        assert_eq!("6", Day1.solve(&String::from("1212"), ProblemPart::B));
        assert_eq!("0", Day1.solve(&String::from("1221"), ProblemPart::B));
        assert_eq!("4", Day1.solve(&String::from("123425"), ProblemPart::B));
        assert_eq!("12", Day1.solve(&String::from("123123"), ProblemPart::B));
        assert_eq!("4", Day1.solve(&String::from("12131415"), ProblemPart::B));
    }
}
