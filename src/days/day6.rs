use std::cmp;
use std::collections::HashMap;

use aoc::*;
use days::ChristmasDay;

pub struct Day6;

impl ChristmasDay for Day6 {
    fn solve(&self, data: &str, prob: ProblemPart) -> String {
        self.solve_for_area(data, prob, 10000)
    }
}

impl Day6 {
    fn solve_for_area(&self, data: &str, prob: ProblemPart, size: usize) -> String {
        let mut coords: Vec<(usize, usize)> = data.lines().map(|l| {
            let xy = l.split(", ").map(|el| el.parse::<usize>().unwrap()).collect::<Vec<usize>>();
            (xy[0], xy[1])
        }).collect();

        let minx = coords.iter().min_by_key(|e| e.0).unwrap();
        let miny = coords.iter().min_by_key(|e| e.1).unwrap();
        // Reset base to 0.
        coords.iter_mut().for_each(|e| {
            e.0 -= minx;
            e.1 -= miny;
        });
        let width = coords.iter().max_by_key(|e| e.0).unwrap() + 1;
        let height = coords.iter().max_by_key(|e| e.1).unwrap() + 1;

        let mut counts = HashMap::new();
        coords.iter().for_each(|p| { counts.insert(p, 0); });
        let mut choices = coords.to_vec();
        let mut valid_points = 0;

        for x in 0..width {
            for y in 0..height {
                match prob {
                    ProblemPart::A => {
                        let mut point = (0, 0);
                        let mut min_dist = width + height;
                        let mut num_match = 0;
                        for &p in coords.iter() {
                            let dist = (cmp::max(x, p.0) - cmp::min(x, p.0)) + (cmp::max(y, p.1) - cmp::min(y, p.1));
                            if dist < min_dist {
                                point = p;
                                min_dist = dist;
                                num_match = 0;
                            } else if dist == min_dist {
                                num_match += 1;
                            }
                        }
                        if num_match > 0 {
                            continue;
                        }

                        match counts.get_mut(&point) {
                            Some(i) => *i += 1,
                            None => {}
                        }

                        if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
                            match choices.iter().position(|&x| x == point) {
                                Some(i) => { choices.remove(i); }
                                None => {}
                            }
                        }
                    },
                    ProblemPart::B => {
                        let total_dist = coords.iter().fold(0, |tot_dist , &p| {
                            tot_dist + (cmp::max(x, p.0) - cmp::min(x, p.0)) + (cmp::max(y, p.1) - cmp::min(y, p.1))
                        });
                        if total_dist < size {
                            valid_points += 1;
                        }
                    }
                }



            }
        }
        match prob {
            ProblemPart::A => choices.iter().map(|c| counts.get(c).unwrap()).max().unwrap().to_string(),
            ProblemPart::B => valid_points.to_string()
        }

    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day6_test1() {
        assert_eq!("17", Day6.solve_a("1, 1
1, 6
8, 3
3, 4
5, 5
8, 9"));
        assert_eq!("16", Day6.solve_for_area("1, 1
1, 6
8, 3
3, 4
5, 5
8, 9", ProblemPart::B, 32));
    }
}