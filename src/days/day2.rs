use aoc::*;
use days::ChristmasDay;

pub struct Day2;

impl ChristmasDay for Day2 {
    fn solve(&self, data: &str, prob: ProblemPart) -> String {
        let lines = data.lines().map(|w| w.chars().collect()).collect::<Vec<Vec<char>>>();

        match prob {
            ProblemPart::A => {
                let (l, r) = lines.iter().fold((0i32, 0i32), |(tw, tr), id| {
                    let mut twi = 0;
                    let mut tri = 0;
                    let mut ids = id.clone();
                    ids.sort_by(|a, b| a.cmp(b));
                    let mut fpos = 0;
                    for idx in 1..id.len() + 1 {
                        if idx == id.len() || ids[fpos] != ids[idx] {
                            let cnt = idx - fpos;
                            fpos = idx;
                            if cnt == 2 { twi = 1; }
                            if cnt == 3 { tri = 1; }
                        }
                    }
                    (tw + twi, tr + tri)
                });
                (l * r).to_string()
            }
            ProblemPart::B => {
                let mut out: Vec<char> = Vec::new();
                for left in lines.clone() {
                    if !out.is_empty() { break; }
                    for right in lines.clone() {
                        let mut edit_distance = 0;
                        for idx in 0..left.len() {
                            if left[idx] != right[idx] { edit_distance += 1; }
                        }
                        if edit_distance == 1 {
                            for idx in 0..left.len() {
                                if left[idx] == right[idx] {
                                    out.push(left[idx]);
                                }
                            }
                            break;
                        }
                    }
                }

                out.iter().collect::<String>()
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day24_test1() {
        assert_eq!("12", Day2.solve_a("abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab"));
        assert_eq!("fgij", Day2.solve_b("abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz"));
    }
}