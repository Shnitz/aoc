use aoc::ProblemPart;

pub fn day1(data: &String, prob: ProblemPart) -> String {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!("3", day1(&String::from("1122"), ProblemPart::A));
        assert_eq!("4", day1(&String::from("1111"), ProblemPart::A));
        assert_eq!("0", day1(&String::from("1234"), ProblemPart::A));
        assert_eq!("9", day1(&String::from("91212129"), ProblemPart::A));
        assert_eq!("6", day1(&String::from("1212"), ProblemPart::B));
        assert_eq!("0", day1(&String::from("1221"), ProblemPart::B));
        assert_eq!("4", day1(&String::from("123425"), ProblemPart::B));
        assert_eq!("12", day1(&String::from("123123"), ProblemPart::B));
        assert_eq!("4", day1(&String::from("12131415"), ProblemPart::B));
    }
}
