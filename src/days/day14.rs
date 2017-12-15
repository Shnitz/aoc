use aoc::*;
use days::ChristmasDay;
use days::day10::Day10;
use std::collections::VecDeque;

pub struct Day14;

impl ChristmasDay for Day14 {
    fn solve(&self, data: &str, prob: ProblemPart) -> String {
        let drive: Vec<Vec<u8>> = (0..128).map(|i| {
            Day10.solve_b(&format!("{}-{}", data, i))
                .chars()
                .map(|hex| u8::from_str_radix(&hex.to_string(), 16).unwrap())
                .collect::<Vec<u8>>()
        }).collect::<Vec<Vec<u8>>>();

        match prob {
            ProblemPart::A => drive.iter().map(|rvec| rvec.iter().map(|v| v.count_ones()).sum::<u32>()).sum::<u32>().to_string(),
            ProblemPart::B => {
                let mut mx: Vec<Vec<String>> = drive.iter().map(|rvec| {
                    let mut tmp = vec![];
                    rvec.iter()
                        .map(|cell| format!("{:04b}", cell))
                        .for_each(|s| {
                            tmp.append(&mut s.chars().map(|c| c.to_string()).collect::<Vec<String>>());
                        });
                    tmp
                }).collect::<Vec<Vec<String>>>();

                let mut group_iter = 2..;
                for ydx in 0..mx.len() {
                    for xdx in 0..128 {
                        if mx[ydx][xdx] == "1" {
                            let g = group_iter.next().unwrap().to_string();
                            let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
                            queue.push_back((ydx, xdx));
                            while let Some((y, x)) = queue.pop_front() {
                                let mut w = x;
                                let mut e = x;
                                while w > 0 && mx[y][w - 1] == "1" { w -= 1; }
                                while e < 127 && mx[y][e + 1] == "1" { e += 1; }
                                for f in w..(e + 1) {
                                    mx[y][f] = g.clone();
                                    if y > 0 && mx[y - 1][f] == "1" {
                                        queue.push_back((y - 1, f));
                                    }
                                    if y < 127 && mx[y + 1][f] == "1" {
                                        queue.push_back((y + 1, f));
                                    }
                                }
                            }
                        }
                    }
                }

                (group_iter.next().unwrap() - 2).to_string()
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day14_test1() {
        assert_eq!("8108", Day14.solve_a("flqrgnkx"));
        assert_eq!("8074", Day14.solve_a("jzgqcdpd"));
        assert_eq!("1242", Day14.solve_b("flqrgnkx"));
        assert_eq!("1212", Day14.solve_b("jzgqcdpd"));
    }
}
