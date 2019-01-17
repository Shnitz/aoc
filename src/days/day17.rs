use aoc::*;
use days::ChristmasDay;
use std::collections::VecDeque;

pub struct Day17;

#[derive(Debug, Copy, Clone)]
struct Coord {
    x1: usize,
    x2: usize,
    y1: usize,
    y2: usize,
}

const SOIL: usize = 0;
const CLAY: usize = 1;
const WATER_STANDING: usize = 2;
const WATER_MOVING: usize = 3;

impl ChristmasDay for Day17 {
    fn solve(&self, data: &str, prob: ProblemPart) -> String {
        let mut map_info = data.lines().map(|l| {
            let data = l.split(|c| c == '=' || c == ',' || c == ' ' || c == '.').collect::<Vec<&str>>();
            let a = data[1].parse::<usize>().unwrap();
            let b1 = data[4].parse::<usize>().unwrap();
            let b2 = data[6].parse::<usize>().unwrap();
            if data[0] == "x" {
                Coord { x1: a, x2: a, y1: b1, y2: b2 }
            } else {
                Coord { x1: b1, x2: b2, y1: a, y2: a }
            }
        }).collect::<Vec<Coord>>();

        let x_off = map_info.iter().min_by_key(|c| c.x1).unwrap().x1 - 1;
        let y_off = map_info.iter().min_by_key(|c| c.y1).unwrap().y1 - 1;
        // Zero out the coords
        map_info.iter_mut().for_each(|c| {
            c.x1 -= x_off;
            c.x2 -= x_off;
            c.y1 -= y_off;
            c.y2 -= y_off;
        });

        let max_x = map_info.iter().max_by_key(|c| c.x2).unwrap().x2 + 1;
        let max_y = map_info.iter().max_by_key(|c| c.y2).unwrap().y2;

        let mut map = vec![vec![SOIL; max_y + 1]; max_x + 1];

        map_info.iter().for_each(|c| {
            for x in c.x1..=c.x2 {
                for y in c.y1..=c.y2 {
                    map[x][y] = CLAY;
                }
            }
        });

        let pos_x = 500 - x_off;
        let pos_y = 0;

        let fill_if_able = |map: &mut Vec<Vec<usize>>, x: usize, y: usize| -> bool {
            let mut left_wall: i32 = -1;
            let mut right_wall: i32 = -1;
            for xl in (0..x).rev() {
                if map[xl][y + 1] != CLAY && map[xl][y + 1] != WATER_STANDING {
                    break;
                } else if map[xl][y] == CLAY {
                    left_wall = xl as i32;
                    break;
                }
            }
            for xr in x..=max_x {
                if map[xr][y + 1] != CLAY && map[xr][y + 1] != WATER_STANDING {
                    break;
                } else if map[xr][y] == CLAY {
                    right_wall = xr as i32;
                    break;
                }
            }

            if left_wall != -1 && right_wall != -1 {
                for xs in left_wall + 1..right_wall {
                    map[xs as usize][y] = WATER_STANDING;
                }
                return true;
            }
            false
        };

        let mut stack: VecDeque<(usize, usize)> = VecDeque::new();
        stack.push_back((pos_x, pos_y));

        loop {
            if let Some(cur) = stack.pop_back() {
                if map[cur.0][cur.1] == SOIL {

                    // If we can go deeper do that first
                    if cur.1 < max_y {
                        match map[cur.0][cur.1 + 1] {
                            SOIL => {
                                stack.push_back(cur);
                                stack.push_back((cur.0, cur.1 + 1));
                            }
                            CLAY => {
                                if !fill_if_able(&mut map, cur.0, cur.1) {
                                    map[cur.0][cur.1] = WATER_MOVING;
                                    if cur.0 > 0 {
                                        stack.push_back((cur.0 - 1, cur.1));
                                    }
                                    if cur.0 < max_x {
                                        stack.push_back((cur.0 + 1, cur.1));
                                    }
                                }
                            }
                            WATER_STANDING => {
                                if !fill_if_able(&mut map, cur.0, cur.1) {
                                    map[cur.0][cur.1] = WATER_MOVING;
                                    if cur.0 > 0 {
                                        stack.push_back((cur.0 - 1, cur.1));
                                    }
                                    if cur.0 < max_x {
                                        stack.push_back((cur.0 + 1, cur.1));
                                    }
                                }
                            }
                            WATER_MOVING => {
                                map[cur.0][cur.1] = WATER_MOVING;
                            }
                            _ => {}
                        }
                    } else {
                        // We've reached the bottom of the map;
                        map[cur.0][cur.1] = WATER_MOVING;
                    }
                }
            } else {
                break;
            }
        }

        // Spring source not counted.
        map[pos_x][pos_y] = 4;

        if prob == ProblemPart::A {
            map.iter().map(|r| r.iter().filter(|i| **i == WATER_STANDING || **i == WATER_MOVING).count()).sum::<usize>().to_string()
        } else {
            map.iter().map(|r| r.iter().filter(|i| **i == WATER_STANDING).count()).sum::<usize>().to_string()
        }
    }
}

impl Day17 {
    fn _print(&self, map: &Vec<Vec<usize>>) {
        for y in 0..map[0].len() {
            for x in 0..map.len() {
                print!("{}", match map[x][y] {
                    SOIL => '.',
                    CLAY => '#',
                    WATER_STANDING => '~',
                    WATER_MOVING => '|',
                    _ => '+'
                });
            }
            println!();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day17_test1() {
        assert_eq!("57", Day17.solve_a("x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504"));
    }
}