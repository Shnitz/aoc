use std::cmp;
use std::collections::HashSet;

use aoc::*;
use days::ChristmasDay;

pub struct Day3;

#[derive(Debug)]
struct Square {
    id: i32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl Square {

    fn overlap_bounds(&self, o: &Square) -> (i32, i32, i32, i32){
        let mx = cmp::max(self.x, o.x);
        let my = cmp::max(self.y, o.y);
        let dx = cmp::min(self.x + self.w, o.x + o.w) - mx;
        let dy = cmp::min(self.y + self.h, o.y + o.h) - my;
        (mx, my, dx, dy)
    }

    fn is_overlap(&self, o: &Square) -> bool {
       let (_, _, dx, dy) = self.overlap_bounds(o);
        dx > 0 && dy > 0
    }

    fn overlap(&self, o: &Square) -> HashSet<(i32, i32)> {
        let mut overlap = HashSet::new();
        let (mx, my, dx, dy) = self.overlap_bounds(o);
        if dx > 0 && dy >  0 {
            for ox in mx..mx + dx {
                for oy in my..my + dy {
                    overlap.insert((ox, oy));
                }
            }
        }

        overlap
    }
}

impl ChristmasDay for Day3 {
    fn solve(&self, data: &str, prob: ProblemPart) -> String {
        let squares = data.lines().map(|l| {
            let args = l.split(|c| c == ' ' || c == ',' || c == 'x' || c == ':' || c == '#')
                .filter_map(|c| c.parse::<i32>().ok()).collect::<Vec<i32>>();
            Square { id: args[0], x: args[1], y: args[2], w: args[3], h: args[4] }
        }).collect::<Vec<Square>>();


        match prob {
            ProblemPart::A => {
                let mut overlap = HashSet::new();
                for (idx, l) in squares.iter().enumerate() {
                    for r in &squares[idx + 1..squares.len()] {
                        for el in l.overlap(r) { overlap.insert(el); }
                    }
                }
                overlap.len().to_string()
            }
            ProblemPart::B => {
                squares.iter()
                    .find(|l| {
                        squares.iter()
                            .filter(|r| r.id != l.id)
                            .all(|r| !l.is_overlap(r))
                    })
                    .unwrap().id.to_string()
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day3_test1() {
        assert_eq!("4", Day3.solve_a("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2"));
        assert_eq!("3", Day3.solve_b("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2"));
    }
}