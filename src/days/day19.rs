use aoc::*;
use days::ChristmasDay;

pub struct Day19;

#[derive(Debug, Clone, Copy)]
enum Dir { N, S, E, W }

impl Dir {
    fn right(self) -> Dir {
        match self {
            Dir::N => Dir::E,
            Dir::E => Dir::S,
            Dir::S => Dir::W,
            Dir::W => Dir::N,
        }
    }
    fn left(self) -> Dir {
        match self {
            Dir::N => Dir::W,
            Dir::E => Dir::N,
            Dir::S => Dir::E,
            Dir::W => Dir::S,
        }
    }
}

#[derive(Debug, Clone)]
struct Point { x: usize, y: usize, }

impl Point {
    fn step(&mut self, dir: &Dir) -> Point {
        let mut p = Point { x: self.x, y: self.y };
        match *dir {
            Dir::N => p.y -= 1,
            Dir::E => p.x += 1,
            Dir::S => p.y += 1,
            Dir::W => p.x -= 1,
        }
        p
    }
}

impl ChristmasDay for Day19 {
    fn solve(&self, data: &str, prob: ProblemPart) -> String {
        let maze: Vec<Vec<char>> = data.lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
        let height = maze.len();
        let width = maze[0].len();

        let loc = |p:&Point| {
            if p.x >= width { Option::None }
            else if p.y >= height { Option::None }
            else { Option::Some(maze[p.y][p.x]) }
        };
        let is_path = |test:char| test == '+' || test == '|' || test == '-' || test.is_alphabetic();

        let mut dir = Dir::S;
        let mut pos = Point { x: maze[0].iter().position(|&c| is_path(c)).unwrap(), y:0 };
        let mut path: Vec<char> = Vec::new();
        let mut steps = 0;

        'main: loop {
            steps += 1;
            let cur = loc(&pos).unwrap();
            if cur.is_alphabetic() {
                path.push(cur);
            }
            for st in &[dir, dir.left(), dir.right()] {
                if let Some(n) = loc(&pos.step(&st)) {
                    if is_path(n) {
                        pos = pos.step(&st);
                        dir = *st;
                        continue 'main;
                    }
                }
            }
            break;
        }

        match prob {
            ProblemPart::A => path.iter().collect::<String>(),
            ProblemPart::B => steps.to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day19_test1() {
        assert_eq!("ABCDEF", Day19.solve_a("     |         \n     |  +--+   \n     A  |  C   \n F---|----E|--+\n     |  |  |  D\n     +B-+  +--+\n"));
        assert_eq!("38", Day19.solve_b("     |         \n     |  +--+   \n     A  |  C   \n F---|----E|--+\n     |  |  |  D\n     +B-+  +--+\n"));
    }
}
