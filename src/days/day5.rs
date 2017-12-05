use aoc::*;
use days::ChristmasDay;

pub struct Day5;

impl ChristmasDay for Day5 {
    fn solve(&self, data: &str, prob: ProblemPart) -> String {
        let mut maze: Vec<i32> = data.lines().map(|i| i.parse::<i32>().unwrap()).collect();
        let mut step = 0;
        let mut idx: usize = 0;
        while idx < maze.len() {
            step += 1;
            let num: i32 = maze[idx];
            maze[idx] = match prob {
                ProblemPart::A => num + 1,
                ProblemPart::B => match num {
                    i if num >= 3 => i - 1,
                    _ => num + 1,
                }
            };
            idx = (idx as i32 + num) as usize;
        }
        step.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day5_test1() {
        assert_eq!("5", Day5.solve_a("0\n3\n0\n1\n-3"));
        assert_eq!("10", Day5.solve_b("0\n3\n0\n1\n-3"));
    }
}
