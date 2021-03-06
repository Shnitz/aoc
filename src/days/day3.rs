use aoc::*;
use days::ChristmasDay;

pub struct Day3;

impl ChristmasDay for Day3 {
    fn solve(&self, data: &str, prob: ProblemPart) -> String {
        let num = data.parse::<i32>().unwrap();
        match prob {
            ProblemPart::A => {
                let mut cnt = 0;
                let mut low = 1;
                let mut high = 1;
                while high < num {
                    cnt = cnt + 1;
                    low = high + 1;
                    high = high + (8 * cnt);
                }
                let side = (high - low + 1) / 4;
                let index = match num - low {
                    0 => 0,
                    _ => (num - low) % side,
                };
                let mid = match side {
                    0 => 0,
                    _ => (side / 2) - 1,
                };
                let step = match index {
                    index if index < mid => mid - index,
                    _ => index - mid,
                };
                (cnt as i32 + step).to_string()
            },
            ProblemPart::B => {
                let mut n =  [[0i32; 100]; 100];
                let (mut x, mut y) = (49, 50);
                n[y][x] = 1;
                n[y][x - 1] = 1;
                let mut dir = 1;
                while n[y][x] <= num {
                    // println!("{},{}\n{},{},{}\n{},{},{}\n{},{},{}", x, y,
                    // n[y - 1][x - 1], n[y - 1][x], n[y - 1][x + 1],
                    // n[y][x - 1], n[y][x], n[y][x + 1],
                    // n[y + 1][x - 1], n[y + 1][x], n[y + 1][x + 1]);
                    match dir {
                        0 => {x = x + 1;},
                        1 => {y = y - 1;},
                        2 => {x = x - 1;},
                        _ => {y = y + 1;},
                    }
                    let should_turn = match dir {
                        0 => n[y - 1][x] == 0,
                        1 => n[y][x - 1] == 0,
                        2 => n[y + 1][x] == 0,
                        _ => n[y][x + 1] == 0,
                    };
                    if should_turn {
                        dir = (dir + 1) % 4;
                    }
                    //Generate next num
                    n[y][x] = n[y - 1][x - 1] + n[y - 1][x] + n[y - 1][x + 1] +
                    n[y][x - 1] + n[y][x] + n[y][x + 1] +
                    n[y + 1][x - 1] + n[y + 1][x] + n[y + 1][x + 1];
                }
                n[y][x].to_string()
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day3_test1() {
        assert_eq!("0", Day3.solve_a("1"));
        assert_eq!("3", Day3.solve_a("12"));
        assert_eq!("2", Day3.solve_a("23"));
        assert_eq!("31", Day3.solve_a("1024"));
        assert_eq!("475", Day3.solve_a("277678"));
        assert_eq!("1", Day3.solve_a("2"));
        assert_eq!("3", Day3.solve_a("16"));
    }

    #[test]
    fn day3_test2() {
        assert_eq!("2", Day3.solve_b("1"));
        assert_eq!("4", Day3.solve_b("2"));
        assert_eq!("10", Day3.solve_b("5"));
        assert_eq!("23", Day3.solve_b("22"));
        assert_eq!("806", Day3.solve_b("750"));
    }
}
