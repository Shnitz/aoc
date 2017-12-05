pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;

use aoc::ProblemPart;

pub trait ChristmasDay {
    fn solve(&self, data: &str, prob: ProblemPart) -> String {
        match prob {
            ProblemPart::A => self.solve_a(data),
            ProblemPart::B => self.solve_b(data),
        }
    }
    fn solve_a(&self, data: &str) -> String {
        self.solve(data, ProblemPart::A)
    }
    fn solve_b(&self, data: &str) -> String {
        self.solve(data, ProblemPart::B)
    }
    fn solve_all(&self, data: String) -> (String, String) {
        let a = self.solve_a(&data.clone());
        let b = self.solve_b(&data);
        (a, b)
    }
}

pub fn run(day: i8, data: String) -> (String, String){
    let fnct: Option<Box<ChristmasDay>> = match day {
        1 => Some(Box::new(day1::Day1)),
        2 => Some(Box::new(day2::Day2)),
        3 => Some(Box::new(day3::Day3)),
        4 => Some(Box::new(day4::Day4)),
        5 => Some(Box::new(day5::Day5)),
        _ => None,
    };
    match fnct {
        Some(x) => x.solve_all(data),
        None => (String::from("NULL"), String::from("NULL")),
    }
}
