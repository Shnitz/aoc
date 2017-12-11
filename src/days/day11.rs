use aoc::*;
use days::ChristmasDay;
use std::cmp;

pub struct Day11;

enum Dir { N, NE, SE, S, SW, NW }
impl Dir {
  fn get(dir: &str) -> Option<Dir> {
    match dir {
      "n" => Some(Dir::N),
      "ne" => Some(Dir::NE),
      "se" => Some(Dir::SE),
      "s" => Some(Dir::S),
      "sw" => Some(Dir::SW),
      "nw" => Some(Dir::NW),
      _ => None,
    }
  }
}

impl ChristmasDay for Day11 {
    fn solve(&self, data: &str, prob: ProblemPart) -> String {
        let steps: Vec<Dir> = data.split(',').map(Dir::get).map(Option::unwrap).collect::<Vec<Dir>>();
        // Use starcraft style 3d coordinates for hexes.
        let mut point = (0i32, 0i32, 0i32);
        let mut dist: i32 = 0;
        for step in steps {
            match step {
                Dir::N => { point.1 += 1; point.2 -= 1; }
                Dir::NE => { point.0 += 1; point.2 -= 1; }
                Dir::SE => { point.0 += 1; point.1 -= 1; }
                Dir::S => { point.1 -= 1; point.2 += 1; }
                Dir::SW  => { point.0 -= 1; point.2 += 1; }
                Dir::NW => { point.0 -= 1; point.1 += 1; }
            }
            let cur_dist = cmp::max(cmp::max(point.0.abs(), point.1.abs()), point.2.abs());
            dist = match prob {
                ProblemPart::A => cur_dist,
                ProblemPart::B => cmp::max(cur_dist, dist),
            };
        }
        dist.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day11_test1() {
        assert_eq!("3", Day11.solve_a("ne,ne,ne"));
        assert_eq!("0", Day11.solve_a("ne,ne,sw,sw"));
        assert_eq!("2", Day11.solve_a("ne,ne,s,s"));
        assert_eq!("3", Day11.solve_a("se,sw,se,sw,sw"));
    }
}
