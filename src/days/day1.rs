use std::collections::HashSet;

use aoc::*;
use days::ChristmasDay;

pub struct Day1;

impl ChristmasDay for Day1 {
    fn solve(&self, data: &str, prob: ProblemPart) -> String {
        let instr = data.split("\n").map(|e| e.trim()).collect::<Vec<&str>>();
        let fix_freq = |num: i32, op: &&str| {
            let mut ch = op.chars();
            let op_code = ch.next().unwrap();
            let val = ch.collect::<String>().parse::<i32>().unwrap();
            match op_code {
                '+' => num + val,
                _ => num - val
            }
        };
        match prob {
            ProblemPart::A =>
                instr.iter().fold(0i32, fix_freq).to_string(),
            ProblemPart::B => {
                let mut idx = 0usize;
                let mut freq = 0i32;
                let mut freqs = HashSet::new();
                while !freqs.contains(&freq) {
                    freqs.insert(freq);
                    freq = fix_freq(freq, &instr[idx]);
                    idx = (idx + 1) % instr.len();
                }
                freq.to_string()
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day24_test1() {
        assert_eq!("3", Day1.solve_a("+1\n-2\n+3\n+1"));
        assert_eq!("3", Day1.solve_a(&"+1, +1, +1".replace(", ", "\n")));
        assert_eq!("0", Day1.solve_a(&"+1, +1, -2".replace(", ", "\n")));
        assert_eq!("-6", Day1.solve_a(&"-1, -2, -3".replace(", ", "\n")));

        assert_eq!("0", Day1.solve_b(&"+1, -1".replace(", ", "\n")));
        assert_eq!("10", Day1.solve_b(&"+3, +3, +4, -2, -4".replace(", ", "\n")));
        assert_eq!("5", Day1.solve_b(&"-6, +3, +8, +5, -6".replace(", ", "\n")));
        assert_eq!("14", Day1.solve_b(&"+7, +7, -2, -7, -4".replace(", ", "\n")));
    }
}