use days::ChristmasDay;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
pub struct Day15;

const A_FACTOR: i64 = 16807;
const B_FACTOR: i64 = 48271;
const DIVISOR: i64 = 2147483647;

fn parse(data: &str) -> (i64, i64) {
    let mut lines = data.lines();
    (lines.next().unwrap().split_whitespace().last().unwrap().parse::<i64>().unwrap(),
    lines.next().unwrap().split_whitespace().last().unwrap().parse::<i64>().unwrap())
}

impl ChristmasDay for Day15 {
    fn solve_a(&self, data: &str) -> String {
        let (mut a, mut b) = parse(data);
        let mut judge = 0;

        for _ in 0..40_000_000 {
            a = (a * A_FACTOR) % DIVISOR;
            b = (b * B_FACTOR) % DIVISOR;
            if a & 0x0000FFFF == b & 0x0000FFFF {
                judge += 1;
            }
        }

        judge.to_string()
    }

    fn solve_b(&self, data: &str) -> String {
        let (a, b) = parse(data);
        let mut judge = 0;
        let (atx, arx): (Sender<i64>, Receiver<i64>) = mpsc::channel();
        let (btx, brx): (Sender<i64>, Receiver<i64>) = mpsc::channel();

        thread::spawn(move || {
            let mut result = a;
            for _ in 0..5_000_000 {
                result = (result * A_FACTOR) % DIVISOR;
                while result % 4 != 0 { result = (result * A_FACTOR) % DIVISOR; }
                atx.send(result).unwrap();
            }
        });
        thread::spawn(move || {
            let mut result = b;
            for _ in 0..5_000_000 {
                result = (result * B_FACTOR) % DIVISOR;
                while result % 8 != 0 { result = (result * B_FACTOR) % DIVISOR; }
                btx.send(result).unwrap();
            }
        });

        for _ in 0..5_000_000 {
            if arx.recv().unwrap() & 0x0000FFFF == brx.recv().unwrap() & 0x0000FFFF {
                judge += 1;
            }
        }

        judge.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day15_test1() {
        assert_eq!("588", Day15.solve_a("Generator A starts with 65
        Generator B starts with 8921"));
        assert_eq!("309", Day15.solve_b("Generator A starts with 65
        Generator B starts with 8921"));
    }
}
