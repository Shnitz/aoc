use aoc::*;
use days::ChristmasDay;
use std::cmp;

pub struct Day5;

fn eq(l: char, r: char) -> bool {
    if l.is_lowercase() { l.to_ascii_uppercase() == r } else { l.to_ascii_lowercase() == r }
}

fn react(mut pol: Vec<char>) -> usize {
    let mut idx = 0;
    while idx + 1 < pol.len() {
        if eq(pol[idx], pol[idx + 1]) {
            pol.drain(idx..idx + 2);
            idx = if idx == 0 { 0 } else { idx - 1 };
        } else {
            idx += 1;
        }
    }
    pol.len()
}

impl ChristmasDay for Day5 {
    fn solve(&self, data: &str, prob: ProblemPart) -> String {
        let pol = data.chars().collect::<Vec<char>>();

        match prob {
            ProblemPart::A => react(pol).to_string(),
            ProblemPart::B => {
                (b'A'..=b'z')
                    .filter_map(|c| {
                        let c = c as char;
                        if c.is_alphabetic() { Some(c) } else { None }
                    }).fold(pol.len(), |min, rem| {
                    let filtered = pol.iter().cloned().filter(|&c| c.to_ascii_lowercase() != rem).collect::<Vec<char>>();
                    cmp::min(min, react(filtered))
                }).to_string()
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day24_test1() {
        assert_eq!("10", Day5.solve_a("dabAcCaCBAcCcaDA"));
        assert_eq!("4", Day5.solve_b("dabAcCaCBAcCcaDA"));
    }
}