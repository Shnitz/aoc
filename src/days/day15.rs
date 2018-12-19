use aoc::*;
use days::ChristmasDay;

pub struct Day15;

#[derive(Debug, Copy, Clone)]
struct Unit {
    x: usize,
    y: usize,
    t: char,
    h: i32,
}

impl ChristmasDay for Day15 {
    fn solve(&self, data: &str, _prob: ProblemPart) -> String {
        let map: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

        let mut units = vec![];

        for y in 0..map.len() {
            for x in 0..map[y].len() {
                match map[y][x] {
                    'E' => units.push(Unit { x, y, t: 'E', h: 200 }),
                    'G' => units.push(Unit { x, y, t: 'G', h: 200 }),
                    _ => {}
                }
            }
        }
        let maze: Vec<Vec<usize>> = map.iter().map(|r| r.iter().map(|e| { if *e == '#' { 1 } else { 0 } }).collect()).collect();
        let maze_h = maze.len();
        let maze_w = maze[0].len();

        maze.iter().for_each(|r| {
            r.iter().for_each(|c| { print!("{}", c); });
            println!();
        });

        let mut round = 0;
        loop {
            println!("{:?}", units);

            // Sort the units again.
            units = units.into_iter().filter(|t| t.h > 0).collect::<Vec<Unit>>();
            units.sort_by(|a, b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)));

            if units.iter().filter(|e| e.t == 'E').count() == 0
                || units.iter().filter(|e| e.t == 'G').count() == 0 {
                break;
            }

            for unit_idx in 0..units.len() {
                let cur_x = units[unit_idx].x;
                let cur_y = units[unit_idx].y;
                let cur_type = units[unit_idx].t;

                // Determine if an attack can be made.
                let mut attacks = units.iter().filter(|t| t.t != cur_type).filter(|t| {
                    cur_x == t.x && cur_y - 1 == t.y
                        || cur_x - 1 == t.x && cur_y == t.y
                        || cur_x + 1 == t.x && cur_y == t.y
                        || cur_x == t.x && cur_y + 1 == t.y
                }).cloned().collect::<Vec<Unit>>();
                if attacks.len() > 0 {
                    // Attack
                    attacks.sort_by(|a, b| a.h.cmp(&b.h).then(a.y.cmp(&b.y)).then(a.x.cmp(&b.x)));
                    units.iter_mut().filter(|t| t.x == attacks[0].x && t.y == attacks[0].y).for_each(|t| { t.h -= 3; });
                } else {
                    // Move

                    // Get the set of candidate attack points.
                    let at_units = units.iter().filter(|u| u.t != cur_type).cloned().collect::<Vec<Unit>>();
                    let mut targets = at_units.iter()
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
                    let mut reachable = self.sorted_shortest_paths(&maze, (cur_x, cur_y), &targets);
                    let min_dist = reachable.iter().min_by_key(|t| t.2).unwrap().2;
                    reachable = reachable.into_iter().filter(|t| t.2 == min_dist).collect::<Vec<(usize, usize, i32)>>();



                    println!("r {:?}", reachable);
//                    let _target = reachable[0];
                }
            }

            if units.iter().all(|u| u.t == 'G') || units.iter().all(|u| u.t == 'E') {
                break;
            }

            round += 1;
        }

        (units.iter().map(|e| e.h).sum::<i32>() * round).to_string()
    }
}

impl Day15 {
    fn sorted_shortest_paths(&self, maze: &Vec<Vec<usize>>, source: (usize, usize), target_list: &Vec<(usize, usize)>) -> Vec<(usize, usize, i32)> {
        let mut targets = target_list.iter().map(|t| (t.0, t.1, -1)).collect::<Vec<(usize, usize, i32)>>();
        let maze_w = maze[0].len();
        let maze_h = maze.len();

        // Reachability
        let mut visited = vec![vec![false; maze_w]; maze_h];
        visited[source.1][source.0] = true;

        let mut q = vec![];
        q.push((source.0, source.1, 0));

        while !q.is_empty() {
            // Dequeue the front cell in the queue and enqueue its adjacent cells
            let (cx, cy, cd) = q.remove(0);
            // Update the distances
            targets.iter_mut().filter(|t| t.0 == cx && t.1 == cy).for_each(|t| t.2 = cd);

            for i in 0..4 {
                let row: i32 = cx as i32 + [-1, 0, 0, 1][i];
                let col: i32 = cy as i32 + [0, -1, 1, 0][i];
                // if adjacent cell is valid, has path and not visited yet, enqueue it.
                if row > 0 && col > 0 {
                    let xpos = col as usize;
                    let ypos = row as usize;
                    if xpos < maze_w && ypos < maze_h
                        && maze[ypos][xpos] == 0 && !visited[ypos][xpos] {
                        // mark cell as visited and enqueue it
                        visited[ypos][xpos] = true;
                        q.push((xpos, ypos, cd + 1));
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day24_test1() {
//        assert_eq!("18740", Day15.solve_a("#########
//#G......#
//#.E.#...#
//#..##..G#
//#...##..#
//#...#...#
//#.G...G.#
//#.....G.#
//#########"));
        assert_eq!("27730", Day15.solve_a("#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######"));
//        assert_eq!("36334", Day15.solve_a("#######
//#G..#E#
//#E#E.E#
//#G.##.#
//#...#E#
//#...E.#
//#######"));
//        assert_eq!("39514", Day15.solve_a("#######
//#E..EG#
//#.#G.E#
//#E.##E#
//#G..#.#
//#..E#.#
//#######"));
//        assert_eq!("27755", Day15.solve_a("#######
//#E.G#.#
//#.#G..#
//#G.#.G#
//#G..#.#
//#...E.#
//#######"));
//        assert_eq!("28944", Day15.solve_a("#######
//#.E...#
//#.#..G#
//#.###.#
//#E#G#G#
//#...#G#
//#######"));

//        assert_eq!("", Day15.solve_b(""));
    }
}