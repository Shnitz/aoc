use aoc::*;
use days::ChristmasDay;

pub struct Day6;

impl ChristmasDay for Day6 {
    fn solve(&self, data: &str, prob: ProblemPart) -> String {
        let mut blocks: Vec<i32> = data.split_whitespace().map(|i| i.parse::<i32>().unwrap()).collect();
        let mut seen: Vec<Vec<i32>> = vec![blocks.clone()];
        let mut step: u32 = 0;
        loop {
            step += 1;
            let (mut idx, max): (usize, i32) = match blocks.iter().enumerate()
                .rev()
                .max_by_key(|&(_, block)| block) {
                    Some((idx, block)) => (idx, *block),
                    None => panic!("No Max found."),
                };
            blocks[idx] = 0;
            for _ in 0..max {
                idx = (idx + 1) % blocks.len();
                blocks[idx] += 1;
            }
            if seen.contains(&blocks) {
                break match prob {
                    ProblemPart::A => step.to_string(),
                    ProblemPart::B => (seen.len() - seen.iter().position(|x| x == &blocks).unwrap()).to_string(),
                }
            }
            seen.push(blocks.clone());
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day6_test1() {
        assert_eq!("5", Day6.solve_a("0 2 7 0"));
        assert_eq!("4", Day6.solve_b("0 2 7 0"));
    }
}
