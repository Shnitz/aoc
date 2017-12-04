use aoc::ProblemPart;

pub fn day4(data: &String, prob: ProblemPart) -> String {
    data.lines()
        .filter(|l| is_valid(*l, &prob))
        .collect::<Vec<_>>()
        .len()
        .to_string()
}

fn is_valid<S>(line: S, _prob: &ProblemPart) -> bool where S: Into<String> {
    let sline: String = line.into();
    let mut words: Vec<String> = sline.split_whitespace().map(String::from).collect::<Vec<String>>();
    match *_prob {
        ProblemPart::A => {},
        ProblemPart::B => {
            words = words.iter().map(|w| {
                let mut chars: Vec<char> = w.chars().collect();
                chars.sort();
                chars.iter().collect::<String>()
            }).collect();
        },
    }
    words.sort();
    let size = &words.len();
    words.dedup();
    *size == words.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(true, is_valid("aa bb cc dd ee", &ProblemPart::A));
        assert_eq!(false, is_valid("aa bb cc dd aa", &ProblemPart::A));
        assert_eq!(true, is_valid("aa bb cc dd aaa", &ProblemPart::A));

        assert_eq!(true, is_valid("abcde fghij", &ProblemPart::B));
        assert_eq!(false, is_valid("abcde xyz ecdab", &ProblemPart::B));
        assert_eq!(true, is_valid("a ab abc abd abf abj", &ProblemPart::B));
        assert_eq!(true, is_valid("iiii oiii ooii oooi oooo", &ProblemPart::B));
        assert_eq!(false, is_valid("oiii ioii iioi iiio", &ProblemPart::B));
    }
}
