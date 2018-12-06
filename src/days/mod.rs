use aoc::ProblemPart;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;

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

pub fn run(day: i8, data: String) -> (String, String) {
    let fnct: Option<Box<ChristmasDay>> = match day {
        1 => Some(Box::new(day1::Day1)),
        2 => Some(Box::new(day2::Day2)),
        3 => Some(Box::new(day3::Day3)),
        4 => Some(Box::new(day4::Day4)),
        5 => Some(Box::new(day5::Day5)),
        6 => Some(Box::new(day6::Day6)),
        7 => Some(Box::new(day7::Day7)),
        8 => Some(Box::new(day8::Day8)),
        9 => Some(Box::new(day9::Day9)),
        10 => Some(Box::new(day10::Day10)),
        11 => Some(Box::new(day11::Day11)),
        12 => Some(Box::new(day12::Day12)),
        13 => Some(Box::new(day13::Day13)),
        14 => Some(Box::new(day14::Day14)),
        15 => Some(Box::new(day15::Day15)),
        16 => Some(Box::new(day16::Day16)),
        17 => Some(Box::new(day17::Day17)),
        18 => Some(Box::new(day18::Day18)),
        19 => Some(Box::new(day19::Day19)),
        20 => Some(Box::new(day20::Day20)),
        21 => Some(Box::new(day21::Day21)),
        22 => Some(Box::new(day22::Day22)),
        23 => Some(Box::new(day23::Day23)),
        24 => Some(Box::new(day24::Day24)),
        _ => None,
    };
    match fnct {
        Some(x) => x.solve_all(data),
        None => (String::from("NULL"), String::from("NULL")),
    }
}
