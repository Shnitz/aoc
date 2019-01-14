use days::ChristmasDay;

pub struct Day15;

#[derive(Debug, Copy, Clone)]
struct Unit {
    x: usize,
    y: usize,
    t: char,
    h: i32,
    id: usize,
    s: i32,
}

impl ChristmasDay for Day15 {
    fn solve_a(&self, data: &str) -> String {
        self.simulate(data).0.to_string()
    }
    fn solve_b(&self, data: &str) -> String {

        for elf_power in 3.. {
            let (score, elves_survive) = self.simulate_elf(data, elf_power);

            if elves_survive {
                return score.to_string();
            }
        }

        "Fail".to_string()
    }
}

impl Day15 {
    fn simulate_elf(&self, data: &str, elf_power: i32) -> (i32, bool) {
        let map: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

        let mut units = vec![];
        let mut id = 0;
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                match map[y][x] {
                    'E' => units.push(Unit { x, y, t: 'E', h: 200, id, s: elf_power }),
                    'G' => units.push(Unit { x, y, t: 'G', h: 200, id, s: 3 }),
                    _ => {}
                }
                id += 1;
            }
        }
        let maze: Vec<Vec<usize>> = map.iter().map(|r| r.iter().map(|e| { if *e == '#' { 1 } else { 0 } }).collect()).collect();
        let maze_h = maze.len();
        let maze_w = maze[0].len();
        let num_elves = units.iter().filter(|u| u.t == 'E').count();

        let attack_options = |units: &Vec<Unit>, &unit_idx: &usize| -> Vec<Unit> {
            units.iter().filter(|t| t.t != units[unit_idx].t).filter(|t| {
                units[unit_idx].x == t.x && units[unit_idx].y - 1 == t.y
                    || units[unit_idx].x - 1 == t.x && units[unit_idx].y == t.y
                    || units[unit_idx].x + 1 == t.x && units[unit_idx].y == t.y
                    || units[unit_idx].x == t.x && units[unit_idx].y + 1 == t.y
            }).cloned().collect::<Vec<Unit>>()
        };

        let mut round = 0;
        loop {

            // Sort the units again.
            units = units.into_iter().filter(|t| t.h > 0).collect::<Vec<Unit>>();
            units.sort_by(|a, b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)));

//            self.print_maze(&maze, &units);

            if units.iter().filter(|e| e.t == 'E').count() == 0
                || units.iter().filter(|e| e.t == 'G').count() == 0 {
                break;
            }

            let mut unit_idx = 0;
            while unit_idx < units.len() {
                let cur_type = units[unit_idx].t;

                if units.iter().all(|u| u.t == 'G') || units.iter().all(|u| u.t == 'E') {
                    let health = units.iter().map(|e| e.h).sum::<i32>();
                    return (health * round, num_elves == units.iter().filter(|u| u.t == 'E').count());
                }

                // Determine if an attack can be made.
                let mut attacks = attack_options(&units, &unit_idx);
                if attacks.len() == 0 {
                    // Move
                    // Get the set of candidate attack points.
                    let at_units = units.iter().filter(|u| u.t != cur_type).cloned().collect::<Vec<Unit>>();
                    let targets = at_units.iter()
                        // Attack points
                        .fold(vec![], |mut points, u| {
                            if u.x > 0 && maze[u.y][u.x - 1] == 0 { points.push((u.x - 1, u.y)) }
                            if u.x < maze_w && maze[u.y][u.x + 1] == 0 { points.push((u.x + 1, u.y)) }
                            if u.y > 0 && maze[u.y - 1][u.x] == 0 { points.push((u.x, u.y - 1)) }
                            if u.y < maze_h && maze[u.y + 1][u.x] == 0 { points.push((u.x, u.y + 1)) }
                            points
                        })
                        .into_iter().filter(|t| units.iter().all(|u| u.x != t.0 || u.y != t.1))
                        .collect::<Vec<(usize, usize)>>();

                    // Determine target.
                    let reachable = self.sorted_shortest_paths(&maze, &units, (units[unit_idx].x, units[unit_idx].y), &targets);
                    let min_to = reachable.iter().cloned().min_by_key(|t| t.2);
                    if let Some(min_t) = min_to {
                        let min_dist = min_t.2;
                        let best_targets = reachable.into_iter().filter_map(|t| if t.2 == min_dist { Some((t.0, t.1)) } else { None }).collect::<Vec<(usize, usize)>>();

                        let mut move_options = (0..4).filter_map(|i| {
                            let xi32: i32 = units[unit_idx].x as i32 + [-1, 0, 0, 1][i];
                            let yi32: i32 = units[unit_idx].y as i32 + [0, -1, 1, 0][i];
                            if xi32 >= 0 && yi32 >= 0 {
                                let x = xi32 as usize;
                                let y = yi32 as usize;
                                if x < maze_w && y < maze_h && maze[y][x] == 0 && units.iter().all(|u| u.x != x || u.y != y) {
                                    let paths = self.sorted_shortest_paths(&maze, &units, (x, y), &best_targets);
                                    if let Some(min_dist) = paths.iter().min_by_key(|t| t.2) {
                                        return Some((x, y, min_dist.2));
                                    }
                                }
                            }
                            None
                        }).collect::<Vec<(usize, usize, i32)>>();
                        move_options.sort_by(|a, b| {
                            a.2.cmp(&b.2).then(a.1.cmp(&b.1)).then(a.0.cmp(&b.1))
                        });
                        //Use the best move option.
                        if let Some(to) = move_options.get(0) {
                            units[unit_idx].x = to.0;
                            units[unit_idx].y = to.1;
                        }
                    }
                }

                attacks = attack_options(&units, &unit_idx);
                if attacks.len() > 0 {
                    // Attack
                    attacks.sort_by(|a, b| a.h.cmp(&b.h).then(a.y.cmp(&b.y)).then(a.x.cmp(&b.x)));

                    if let Some(idx) = units.iter().position(|t| t.x == attacks[0].x && t.y == attacks[0].y) {
                        units[idx].h -= units[unit_idx].s;
                        if units[idx].h <= 0 {
                            units.remove(idx);
                            if idx <= unit_idx {
                                continue;
                            }
                        }
                    }
                }

                unit_idx += 1;
            }

            round += 1;

            if units.iter().all(|u| u.t == 'G') || units.iter().all(|u| u.t == 'E') {
                break;
            }
        }

        let health = units.iter().map(|e| e.h).sum::<i32>();
        (health * round, num_elves == units.iter().filter(|u| u.t == 'E').count())
    }

    fn simulate(&self, data: &str) -> (i32, bool) {
        self.simulate_elf(data, 3)
    }

    fn sorted_shortest_paths(&self, maze: &Vec<Vec<usize>>, blocks: &Vec<Unit>, source: (usize, usize), target_list: &Vec<(usize, usize)>) -> Vec<(usize, usize, i32)> {
        let mut targets = target_list.iter().map(|t| (t.0, t.1, -1)).collect::<Vec<(usize, usize, i32)>>();
        let maze_w = maze[0].len();
        let maze_h = maze.len();

        // Reachability
        let mut visited = vec![vec![false; maze_w]; maze_h];
        visited[source.1][source.0] = true;
        blocks.iter().for_each(|p| { visited[p.y][p.x] = true });

        let mut q = vec![];
        q.push((source.0, source.1, 0));

        while !q.is_empty() {
            // Dequeue the front cell in the queue and enqueue its adjacent cells
            let (cx, cy, cd) = q.remove(0);
            // Update the distances
            targets.iter_mut().filter(|t| t.0 == cx && t.1 == cy).for_each(|t| if t.2 == -1 || t.2 > cd { t.2 = cd });

            for i in 0..4 {
                let xi32: i32 = cx as i32 + [-1, 0, 0, 1][i];
                let yi32: i32 = cy as i32 + [0, -1, 1, 0][i];
                // if adjacent cell is valid, has path and not visited yet, enqueue it.
                if xi32 > 0 && yi32 > 0 {
                    let x = xi32 as usize;
                    let y = yi32 as usize;
                    if x < maze_w && y < maze_h
                        && maze[y][x] == 0 && !visited[y][x] {
                        // mark cell as visited and enqueue it
                        visited[y][x] = true;
                        q.push((x, y, cd + 1));
                    }
                }
            }
        }

        let mut reachable = targets.into_iter().filter(|t| t.2 >= 0).collect::<Vec<(usize, usize, i32)>>();
        reachable.sort_by(|a, b| {
            a.2.cmp(&b.2).then(a.1.cmp(&b.1)).then(a.0.cmp(&b.0))
        });

        reachable
    }

    fn _print_maze(&self, maze: &Vec<Vec<usize>>, units: &Vec<Unit>) {
        let mut img = maze.iter().map(|r| {
            r.iter().map(|c| if *c == 0 { ".".to_string() } else { "#".to_string() }).collect::<Vec<String>>()
        }).collect::<Vec<Vec<String>>>();
        units.iter().for_each(|u| img[u.y][u.x] = u.t.to_string());
        img.iter().map(|l| l.concat()).enumerate().for_each(|(idx, l)| {
            print!("{}", l);
            units.iter().for_each(|unit| if unit.y == idx { print!(" {}({}),", unit.t, unit.h) });
            println!();
        });
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day15_test1() {
        assert_eq!("18740", Day15.solve_a("#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########"));
    }

    #[test]
    fn day15_test2() {
        assert_eq!("27730", Day15.solve_a("#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######"));
    }

    #[test]
    fn day15_test3() {
        assert_eq!("36334", Day15.solve_a("#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######"));
    }

    #[test]
    fn day15_test4() {
        assert_eq!("39514", Day15.solve_a("#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######"));
    }

    #[test]
    fn day15_test5() {
        assert_eq!("27755", Day15.solve_a("#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######"));
    }

    #[test]
    fn day15_test6() {
        assert_eq!("28944", Day15.solve_a("#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######"));
    }

    #[test]
    fn day15_test7() {
        assert_eq!("4988", Day15.solve_b("#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######"));
    }

    #[test]
    fn day15_test8() {
        assert_eq!("31284", Day15.solve_b("#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######"));
    }

    #[test]
    fn day15_test9() {
        assert_eq!("3478", Day15.solve_b("#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######"));
    }

    #[test]
    fn day15_test10() {
        assert_eq!("6474", Day15.solve_b("#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######"));
    }


    #[test]
    fn day15_test11() {
        assert_eq!("1140", Day15.solve_b("#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########"));
    }

//        assert_eq!("", Day15.solve_b(""));
}