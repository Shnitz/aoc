use days::ChristmasDay;

pub struct Day17;

impl ChristmasDay for Day17 {
    fn solve_a(&self, data: &str) -> String {
        let step: usize = data.parse::<usize>().unwrap();
        let mut buf: Vec<usize> = vec![0];
        let mut pos = 0;
        for idx in 1..2018 {
            pos = (pos + step) % idx + 1;
            buf.insert(pos, idx);
        }

        buf[pos + 1].to_string()
    }

    fn solve_b(&self, data: &str) -> String {
        let step: usize = data.parse::<usize>().unwrap();
        // [ 0 ]
        let mut pos_0 = 0;
        let mut after_0 = 0;
        let mut pos = 0;

        for idx in 1..50_000_000 {
            pos = (pos + step) % idx + 1;
            match pos {
                x if x <= pos_0 => { pos_0 += 1; },
                x if x == pos_0 + 1 => after_0 = idx,
                _ => {},
            }
        }

        after_0.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day17_test1() {
        assert_eq!("638", Day17.solve_a("3"));
        assert_eq!("926", Day17.solve_a("394"));
    }
}
