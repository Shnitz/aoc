use aoc::*;
use days::ChristmasDay;

pub struct Day15;

struct Unit {
    x: usize,
    y: usize,
    t: char,
    h: usize,
}

impl ChristmasDay for Day15 {
    fn solve(&self, data: &str, _prob: ProblemPart) -> String {
        let mut map: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

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
        let maze: Vec<Vec<usize>> = map.iter().map(|r| r.iter().map(|e| { if *e == '#' || *e == 'E' || *e == 'G' { 1 } else { 0 } }).collect()).collect();
        let maze_h = maze.len();
        let maze_w = maze[0].len();

        let mut round = 0;
        loop {
            // Sort the units again.
            units.sort_by(|a, b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)));

            if units.iter().filter(|e| e.t == 'E').count() == 0
                || units.iter().filter(|e| e.t == 'G').count() == 0 {
                break;
            }

            for unit_idx in 0..units.len() {
                let cur_x = units[unit_idx].x;
                let cur_y = units[unit_idx].y;
                let cur_type = units[unit_idx].t;

                // Get the set of candidate attack points.
                let targets = units.iter()
                    // Enemies
                    .filter(|u| u.t != cur_type)
                    // Attack points
                    .fold(vec![], |mut points, u| {
                        if u.x > 0 && maze[u.y][u.x - 1] == 0 { points.push((u.x, u.y)) }
                        if u.x < maze_w && maze[u.y][u.x + 1] == 0 { points.push((u.x, u.y)) }
                        if u.y > 0 && maze[u.y - 1][u.x] == 0 { points.push((u.x, u.y)) }
                        if u.y < maze_h && maze[u.y + 1][u.x] == 0 { points.push((u.x, u.y)) }
                        points
                    });

                // Reachability
                let mut visited = vec![vec![false; maze_w]; maze_h];
                visited[cur_y][cur_x] = true;

                let mut q = vec![];
                q.push((*cur_x, *cur_y, 0));

                while !q.is_empty() {
                    let cx = que[0].0;
                    let cy = que[0].1;
                    let cd = que[0].2;

                    // Otherwise dequeue the front cell in the queue and enqueue its adjacent cells
                    q.pop();

                    for i in 0..4 {
                        let row = cx + [-1, 0, 0, 1][i];
                        let col = cy + [0, -1, 1, 0][i];
                        // if adjacent cell is valid, has path and not visited yet, enqueue it.
                        if row >= 0 && row < maze_w && col >= 0 && col < maze_h
                            && maze[row][col] == 0 && !visited[row][col] {
                            // mark cell as visited and enqueue it
                            visited[row][col] = true;
                            q.push((col, row, cd + 1));
                        }
                    }
                }

            }

            round += 1;
        }

        (units.iter().map(|e| e.h).sum::<usize>() * round).to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day24_test1() {
        assert_eq!("18740", Day15.solve_a("#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########"));
        assert_eq!("27730", Day15.solve_a("#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######"));
        assert_eq!("36334", Day15.solve_a("#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######"));
        assert_eq!("39514", Day15.solve_a("#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######"));
        assert_eq!("27755", Day15.solve_a("#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######"));
        assert_eq!("28944", Day15.solve_a("#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######"));

//        assert_eq!("", Day15.solve_b(""));
    }
}