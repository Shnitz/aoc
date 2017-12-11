use aoc::*;
use days::ChristmasDay;

pub struct Day10;

impl ChristmasDay for Day10 {
    fn solve(&self, _data: &str, _prob: ProblemPart) -> String {
        "".to_string()
    }
}

fn twist(length: usize, data: &str, _prob: ProblemPart) -> String {
    let mut list: Vec<usize> = vec!(0; length);
    for x in 0..length { list[x as usize] = x; }
    let mut pos = 0;
    let mut skip = 0;
    let lengths = data.split_whitespace().map(|n| n.trim_matches(',').parse::<usize>().unwrap()).collect::<Vec<usize>>();

    println!("{:?}", list);

    "".to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day10_test1() {
        assert_eq!("12", twist(5, "3, 4, 1, 5", ProblemPart::A));
        // assert_eq!("", Day10.solve_b(""));
    }
}
