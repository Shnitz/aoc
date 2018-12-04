use std::collections::HashMap;
use std::str::FromStr;

use aoc::ProblemPart;
use days::ChristmasDay;

pub struct Day4;

#[derive(Debug, Copy, Clone)]
enum State {
    Sleep,
    Wakeup,
    Startshift,
}

impl FromStr for State {
    type Err = ();
    fn from_str(s: &str) -> Result<State, ()> {
        match s {
            "asleep" => Ok(State::Sleep),
            "up" => Ok(State::Wakeup),
            _ => Ok(State::Startshift),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Log<'a> {
    id: usize,
    date: &'a str,
    h: usize,
    m: usize,
    action: State,
}

impl ChristmasDay for Day4 {
    fn solve(&self, data: &str, prob: ProblemPart) -> String {
        let mut times = data.lines().map(|l| {
            let c = l.split(|c| c == ' ' || c == ':' || c == ']').collect::<Vec<&str>>();
            Log {
                id: c[5].replace("#", "").parse::<usize>().unwrap_or(0),
                date: c[0],
                h: c[1].parse::<usize>().unwrap(),
                m: c[2].parse::<usize>().unwrap(),
                action: c[5].parse::<State>().unwrap(),
            }
        }).collect::<Vec<Log>>();
        times.sort_by(|l, r|
            l.date.cmp(&r.date).then(l.h.cmp(&r.h)).then(l.m.cmp(&r.m)));

        let mut sched: HashMap<usize, (usize, [usize; 60])> = HashMap::new();
        let mut id = 0;
        let mut lt = (0, 0);
        for log in times.clone() {
            match log.action {
                State::Startshift => id = log.id,
                State::Sleep => lt = (log.h, log.m),
                State::Wakeup => {
                    let mins = log.m - lt.1;
                    let start = if lt.0 == 0 { lt.1 } else { 0 };
                    let (sum, overlap) = sched.entry(id).or_insert_with(|| (0, [0; 60]));
                    *sum += mins;
                    overlap[start..start + mins].iter_mut().for_each(|e| *e += 1);
                }
            }
        }

        match prob {
            ProblemPart::A => {
                let (worst_guard, (_, overlap)) = sched.iter().max_by(|(_, (l, _)), (_, (r, _))| l.cmp(r)).unwrap();
                let (best_minute, _) = overlap.iter().enumerate().max_by_key(|a| a.1).unwrap();
                (worst_guard * best_minute).to_string()
            }
            ProblemPart::B => {
                let (worst_guard, _, best_minute) = sched.iter().fold((0, 0, 0), |top, (gid, (_, overlap))| {
                    let (best_minute, max) = overlap.iter().enumerate().max_by_key(|a| a.1).unwrap();
                    if *max > top.1 {
                        (*gid, *max, best_minute)
                    } else {
                        top
                    }
                });

                (worst_guard * best_minute).to_string()
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day4_test1() {
        let input = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:24] falls asleep
[1518-11-05 00:55] wakes up
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-03 00:29] wakes up
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:45] falls asleep";
        assert_eq!("240", Day4.solve_a(input));
        assert_eq!("4455", Day4.solve_b(input));
    }
}