use aoc::*;
use days::ChristmasDay;

pub struct Day2;

impl ChristmasDay for Day2 {
    fn solve(&self, data: &String, prob: ProblemPart) -> String {
        data.lines()
            .fold(0, |acc, l| {
                let mut nums: Vec<_> = l.split_whitespace().map(|i| i.parse::<i32>().unwrap()).collect();
                match prob {
                    ProblemPart::A => acc + nums.iter().max().unwrap() - nums.iter().min().unwrap(),
                    ProblemPart::B => {
                        let mut tot = 0;
                        let size = nums.len();
                        nums.sort();
                        for idx in 0..size {
                            for test in (idx + 1)..size {
                                if nums[test] % nums[idx] == 0 {
                                    tot += nums[test] / nums[idx];
                                }
                            }
                        }
                        acc + tot
                    }
                }
            })
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!("18", Day2.solve(&String::from(
"5 1 9 5
7 5 3
2 4 6 8"), ProblemPart::A));
    }

    #[test]
    fn test2() {
        assert_eq!("9", Day2.solve(&String::from(
"5 9 2 8
    9 4 7 3
    3 8 6 5"), ProblemPart::B));
    }

}
