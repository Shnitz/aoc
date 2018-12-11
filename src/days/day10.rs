use aoc::*;
use days::ChristmasDay;
use std::iter::FromIterator;

pub struct Day10;

impl ChristmasDay for Day10 {
    fn solve(&self, data: &str, prob: ProblemPart) -> String {
        let mut points = data.lines().map(|line| {
            let nums = line.split(|l| l == '<' || l == ',' || l == '>' || l == ' ')
                .filter_map(|c| c.parse::<i32>().ok()).collect::<Vec<i32>>();
            (nums[0], nums[1], nums[2], nums[3])
        }).collect::<Vec<(i32, i32, i32, i32)>>();

        let mut last_size = std::usize::MAX;
        let mut time = 0;
        let mut image: Vec<char>;
        loop {
            let min_x = points.iter().min_by_key(|(x, _, _, _)| x).unwrap().0;
            let min_y = points.iter().min_by_key(|(_, y, _, _)| y).unwrap().1;
            let max_x = points.iter().max_by_key(|(x, _, _, _)| x).unwrap().0;
            let max_y = points.iter().max_by_key(|(_, y, _, _)| y).unwrap().1;
            let width: usize = (max_x - min_x) as usize + 1;
            let height: usize = (max_y - min_y) as usize + 1;

            // loop until all points have an adjacent point.
            if last_size < width * height {
                points.iter_mut().for_each(|(x, y, vx, vy)| {
                    *x -= *vx;
                    *y -= *vy;
                });
                let min_x = points.iter().min_by_key(|(x, _, _, _)| x).unwrap().0;
                let min_y = points.iter().min_by_key(|(_, y, _, _)| y).unwrap().1;
                let max_x = points.iter().max_by_key(|(x, _, _, _)| x).unwrap().0;
                let max_y = points.iter().max_by_key(|(_, y, _, _)| y).unwrap().1;
                let width: usize = (max_x - min_x) as usize + 1;
                let height: usize = (max_y - min_y) as usize + 1;
                image = vec!['.'; (width * height) as usize];

                points.iter().map(|(x, y, _, _)| (x - min_x, y - min_y))
                    .for_each(|(x, y)| {
                        image[x as usize + width * y as usize] = '#';
                    });
                for slice in 0..height {
                    image.insert(slice * width + width + slice, '\n');
                }
                break;
            }
            last_size = width * height;
            time += 1;
            points.iter_mut().for_each(|(x, y, vx, vy)| {
                *x += *vx;
                *y += *vy;
            });
        }

        match prob {
            ProblemPart::A => String::from_iter(image),
            ProblemPart::B => (time - 1).to_string()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day10_test1() {
        assert_eq!("#...#..###
#...#...#.
#...#...#.
#####...#.
#...#...#.
#...#...#.
#...#...#.
#...#..###
", Day10.solve_a("position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>"));
        assert_eq!("3", Day10.solve_b("position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>"));
    }
}