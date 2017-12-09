use aoc::*;
use days::ChristmasDay;

pub struct Day9;

enum Mode { READ, GARBAGE }

impl ChristmasDay for Day9 {
    fn solve(&self, data: &str, prob: ProblemPart) -> String {
        let mut depth = 0;
        let mut mode: Mode = Mode::READ;
        let mut chs = data.chars();
        let mut count = 0;

        while let Some(ch) = chs.next() {
            match mode {
                Mode::READ => {
                    match ch {
                        '{' => depth += 1,
                        '}' => {
                            if prob == ProblemPart::A {
                                count += depth;
                            }
                            depth -= 1;
                        },
                        '<' => mode = Mode::GARBAGE,
                        _ => {},
                    }
                },
                Mode::GARBAGE => {
                    match ch {
                        '>' => mode = Mode::READ,
                        '!' => { chs.next(); },
                        _ => {
                            if prob == ProblemPart::B {
                                count += 1;
                            }
                        },
                    }
                }
            }

        }

        count.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day9_test1() {
        assert_eq!("1", Day9.solve_a("{}"));
        assert_eq!("6", Day9.solve_a("{{{}}}"));
        assert_eq!("5", Day9.solve_a("{{},{}}"));
        assert_eq!("16", Day9.solve_a("{{{},{},{{}}}}"));
        assert_eq!("1", Day9.solve_a("{<a>,<a>,<a>,<a>}"));
        assert_eq!("9", Day9.solve_a("{{<ab>},{<ab>},{<ab>},{<ab>}}"));
        assert_eq!("9", Day9.solve_a("{{<!!>},{<!!>},{<!!>},{<!!>}}"));
        assert_eq!("3", Day9.solve_a("{{<a!>},{<a!>},{<a!>},{<ab>}}"));

        assert_eq!("0", Day9.solve_b("<>"));
        assert_eq!("17", Day9.solve_b("<random characters>"));
        assert_eq!("3", Day9.solve_b("<<<<>"));
        assert_eq!("2", Day9.solve_b("<{!>}>"));
        assert_eq!("0", Day9.solve_b("<!!>"));
        assert_eq!("0", Day9.solve_b("<!!!>>"));
        assert_eq!("10", Day9.solve_b("<{o\"i!a,<{i<a>"));
    }
}
