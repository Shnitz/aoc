use aoc::*;
use days::ChristmasDay;

pub struct Day12;

const PAD: usize = 30;

impl ChristmasDay for Day12 {
    fn solve(&self, data: &str, prob: ProblemPart) -> String {
        let mut lines = data.lines();

        let mut nodes = [0u8; 32];

        let mut state = lines.next().unwrap().split_whitespace()
            .last().unwrap().chars().map(|c| match c {
            '#' => 1,
            _ => 0
        }).collect::<Vec<u8>>();

        lines.next();

        lines.for_each(|l| {
            let instr = l.split_whitespace().collect::<Vec<&str>>();
            let key = instr[0].chars().map(|c| if c == '#' { 1 } else { 0 }).collect::<Vec<u8>>();
            nodes[(key[0] + key[1] * 2 + key[2] * 4 + key[3] * 8 + key[4] * 16) as usize] = if instr[2] == "#" { 1 } else { 0 };
        });

        let mut zero_idx: usize = 0;
        let mut state_len = state.len();
        let mut score = 0;
        let mut diff = 0;
        let mut score_cnt = 0;

        for n in 0i64..match prob {
            ProblemPart::A => 20,
            ProblemPart::B => 50_000_000_000
        } {
            //Ensure empty sides
            if state[0] != 0 || state[1] != 0 || state[2] != 0 {
                state.splice(..0, vec![0; PAD]);
                zero_idx += PAD;
                state_len += PAD;
            }

            if state.last() != Some(&0) || state[state_len - 2] != 0 || state[state_len - 3] != 0 {
                state.append(&mut vec![0u8; PAD]);
                state_len += PAD;
            }
            let mut l2_plant = 0;
            let mut l1_plant = 0;
            let mut l0_plant = 0;
            for idx in 2..state_len - 2 {
                if idx as i32 - 3 > 0 { state[idx - 3] = l2_plant; }
                l2_plant = l1_plant;
                l1_plant = l0_plant;
                let key = &state[idx - 2..=idx + 2];
                l0_plant = nodes[(key[0] + key[1] * 2 + key[2] * 4 + key[3] * 8 + key[4] * 16) as usize];
            }
            state[state_len - 3] = l0_plant;
            state[state_len - 4] = l1_plant;
            state[state_len - 5] = l2_plant;

            let mut offset = 0i64 - zero_idx as i64 - 1;
            let nscore = state.iter().fold(0, |sum, &p| {
                offset += 1;
                sum + p as i64 * offset
            });

            if diff == nscore - score {
                score_cnt += 1;
            } else {
                diff = nscore - score;
                score_cnt = 0;
            }
            score = nscore;
            if score_cnt > 10 {
                let iter_left = 50_000_000_000 - n - 1;
                score += iter_left * diff;
                break;
            }

            if n % 1_000 == 0 { println!("{}", n); }
        }
        if prob == ProblemPart::A {
            let mut offset = 0i64 - zero_idx as i64 - 1;
            score = state.iter().fold(0, |sum, &p| {
                offset += 1;
                sum + p as i64 * offset
            });
        }
        score.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day24_test1() {
        assert_eq!("325", Day12.solve_a("initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #"));
    }
}