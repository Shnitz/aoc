use aoc::*;
use days::ChristmasDay;
use std::collections::HashMap;
pub struct Day22;

enum State { W, F, I }

impl ChristmasDay for Day22 {
    fn solve(&self, data: &str, prob: ProblemPart) -> String {
        infect(data, match prob {
            ProblemPart::A => 10_000,
            ProblemPart::B => 10_000_000,
        }, prob)
    }
}

fn infect(data: &str, runs: u32, prob: ProblemPart) -> String {
    // let mut infected: Vec<(i32, i32, State)> = Vec::new();
    let mut infected: HashMap<(i32, i32), State> = HashMap::new();
    let mut infect_cnt = 0;
    let lines = data.lines().collect::<Vec<&str>>();
    let size = lines.len();
    for y in 0..size {
        let chrs = lines[y].chars().collect::<Vec<char>>();
        for x in 0..chrs.len() {
            if chrs[x] == '#' {
                infected.insert((x as i32, (size as i32) - 1 - y as i32), State::I);
            }
        }
    }
    let mut pos = (((lines.len() / 2)) as i32, ((lines.len() / 2)) as i32);
    //    0
    //  3   1
    //    2
    let mut dir = 0;

    for _ in 0..runs {
        if infected.contains_key(&pos) {
            match prob {
                ProblemPart::A => { infected.remove(&pos); dir = (dir + 1) % 4; },
                ProblemPart::B => {
                    let val = infected.remove(&pos).unwrap();
                    match val {
                        State::W => { infected.insert(pos, State::I); infect_cnt += 1; },
                        State::F => { dir = (dir + 2) % 4; },
                        State::I => { infected.insert(pos, State::F); dir = (dir + 1) % 4; },
                    }
                },
            }
        } else {
            match prob {
                ProblemPart::A => { infected.insert((pos.0, pos.1), State::I); infect_cnt += 1; },
                ProblemPart::B => { infected.insert((pos.0, pos.1), State::W); },
            }
            dir = (dir + 3) % 4;
        }
        match dir {
            0 => { pos.1 += 1; },
            1 => { pos.0 += 1; },
            2 => { pos.1 -= 1; },
            3 => { pos.0 -= 1; },
            _ => { panic!("Unknown Direction.");},
        }
    }

    infect_cnt.to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day22_test1() {
        assert_eq!("5", infect("..#
#..
...", 7, ProblemPart::A));
assert_eq!("41", infect("..#
#..
...", 70, ProblemPart::A));
assert_eq!("5587", infect("..#
#..
...", 10_000, ProblemPart::A));
assert_eq!("26", infect("..#
#..
...", 100, ProblemPart::B));
assert_eq!("2511944", infect("..#
#..
...", 10_000_000, ProblemPart::B));

    }
}
