use aoc::*;
use days::ChristmasDay;
use std::collections::HashSet;

pub struct Day13;

#[derive(Debug, Copy, Clone)]
struct Cart {
    x: usize,
    y: usize,
    dir: usize,
    cx: usize,
    ok: bool,
}

impl ChristmasDay for Day13 {
    fn solve(&self, data: &str, prob: ProblemPart) -> String {
        let mut map: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
        let mut cars = vec![];

        //0 N, 1 E, 2 S, 3 W
        for y1 in 0usize..map.len() {
            for x1 in 0usize..map[y1].len() {
                match map[y1][x1] {
                    '^' => cars.push(Cart { x: x1, y: y1, dir: 0, cx: 0, ok: true }),
                    '>' => { cars.push(Cart { x: x1, y: y1, dir: 1, cx: 0, ok: true }); }
                    'v' => { cars.push(Cart { x: x1, y: y1, dir: 2, cx: 0, ok: true }); }
                    '<' => { cars.push(Cart { x: x1, y: y1, dir: 3, cx: 0, ok: true }); }
                    _ => {}
                }
            }
        }
        cars.iter().for_each(|car| {
            map[car.y][car.x] = if car.dir == 0 || car.dir == 2 { '|' } else { '-' };
        });
//
//        for y in 0usize..map.len() {
//            for x in 0usize..map[y].len() {
//                print!("{}", map[y][x]);
//            }
//            println!();
//        }

        let mut cnt = 0;
        loop {
            cnt += 1;
            if cnt > 16981 {
                break;
            }
            cars = cars.iter().filter_map(|c| if !c.ok { None } else { Some(*c) }).collect::<Vec<Cart>>();
            if cars.len() == 1 && prob == ProblemPart::B {
                return format!("{},{}", cars[0].x, cars[0].y);
            }
            cars.sort_by(|a, b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)));

            let mut move_idx = 0;
            while move_idx < cars.len() {
                let mut x = cars[move_idx].x;
                let mut y = cars[move_idx].y;
                let mut direction = cars[move_idx].dir;
                let mut choice = cars[move_idx].cx;
                //0 N, 1 E, 2 S, 3 W
                match direction {
                    0 => { y -= 1; }
                    1 => { x += 1; }
                    2 => { y += 1; }
                    _ => { x -= 1; }
                }

                direction = match map[y][x] {
                    '|' => { direction }
                    '-' => { direction }
                    '/' => match direction {
                        0 => 1,
                        1 => 0,
                        2 => 3,
                        _ => 2
                    },
                    '\\' => match direction {
                        0 => 3,
                        1 => 2,
                        2 => 1,
                        _ => 0
                    },
                    '+' => {
                        let res = match choice {
                            //Left
                            0 => match direction {
                                0 => 3,
                                1 => 0,
                                2 => 1,
                                _ => 2
                            },
                            // Straight
                            1 => direction,
                            // Right
                            2 => match direction {
                                0 => 1,
                                1 => 2,
                                2 => 3,
                                _ => 0
                            },
                            _ => { panic!("Invalid."); }
                        };
                        choice = (choice + 1) % 3;
                        res
                    }
                    _ => { panic!("Unknown last coord! '{}' at {} {}", map[y][x], x, y); }
                };
                cars[move_idx].x = x;
                cars[move_idx].y = y;
                cars[move_idx].dir = direction;
                cars[move_idx].cx = choice;

                move_idx += 1;
            };
            let mut uniq = HashSet::new();
            let mut idx = 0;
            while idx < cars.len() {
                let x = cars[idx].x;
                let y = cars[idx].y;
                let is_dupe = !uniq.insert((x, y));
                if is_dupe {
//                    println!("{:?}", cars);
                    if prob == ProblemPart::A {
                        return format!("{},{}", x, y);
                    }
                    cars.iter_mut().for_each(|car| {
                        if car.x == x && car.y == y {
                            car.ok = false;
                        }
                    });
                }
                idx += 1;
            }
        }
        "ffff".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day13_test1() {
        assert_eq!("7,3", Day13.solve_a("/->-\\
|   |  /----\\
| /-+--+-\\  |
| | |  | v  |
\\-+-/  \\-+--/
  \\------/   "));
    }

    #[test]
    fn day13_test2() {
        assert_eq!("6,4", Day13.solve_b("/>-<\\
|   |
| /<+-\\
| | | v
\\>+</ |
  |   ^
  \\<->/"));
    }
}