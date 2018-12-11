use aoc::*;
use days::ChristmasDay;

pub struct Day11;

impl ChristmasDay for Day11 {
    fn solve(&self, data: &str, prob: ProblemPart) -> String {
        let serial = data.parse::<i32>().unwrap();

        let mut max_val = 0;
        let mut max_loc = (0, 0, 0);

        let mut grid = vec![[0i32; 301]; 301];
        for y in 0..301 {
            for x in 0..301 {
                grid[x][y] = self.cell_val(x as i32, y as i32, serial);
            }
        }

        if prob == ProblemPart::A {
            for y in 1..298 {
                for x in 1..298 {
                    let mut pow = 0;
                    for xi in 0..3 {
                        for yi in 0..3 {
                            pow += grid[x + xi][y + yi];
                        }
                    }
                    if pow > max_val {
                        max_val = pow;
                        max_loc = (x, y, 0);
                    }
                }
            }
            format!("{},{}", max_loc.0, max_loc.1).to_string()
        } else {
            for y in 1..300 {
                for x in 1..300 {
                    let mut pow = 0;
                    for size in 1..(301 - y).min(301 - x) {
                        let y_edge = y + size - 1;
                        let x_edge = x + size - 1;
                        for i in 0..size {
                            pow += grid[x + i][y_edge] + grid[x_edge][y + i];
                        }
                        pow -= grid[x_edge][y_edge];

                        if pow > max_val {
                            max_val = pow;
                            max_loc = (x, y, size);
                        }
                    }
                }
            }
            format!("{},{},{}", max_loc.0, max_loc.1, max_loc.2).to_string()
        }
    }
}

impl Day11 {
    fn cell_val(&self, x: i32, y: i32, serial: i32) -> i32 {
        let rid = x + 10;
        let mut power = rid * y;
        power += serial;
        power *= rid;
        power = (power / 100) % 10;
        power -= 5;
        power
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day11_test1() {
        assert_eq!(4, Day11.cell_val(3, 5, 8));
        assert_eq!(-5, Day11.cell_val(122, 79, 57));
        assert_eq!(0, Day11.cell_val(217, 196, 39));
        assert_eq!(4, Day11.cell_val(101, 153, 71));
    }

    #[test]
    fn day11_test2() {
        assert_eq!("33,45", Day11.solve_a("18"));
        assert_eq!("21,61", Day11.solve_a("42"));
    }

    //TODO Takes 2 minutes still.
//    #[test]
//    fn day11_test3() {
//        assert_eq!("90,269,16", Day11.solve_b("18"));
//        assert_eq!("232,251,12", Day11.solve_b("42"));
//    }
}