use aoc::*;
use days::ChristmasDay;
use std::collections::VecDeque;

pub struct Day9;

impl ChristmasDay for Day9 {
    fn solve(&self, data: &str, prob: ProblemPart) -> String {
        let game_info = data.split_whitespace().filter_map(|e| e.parse::<usize>().ok()).collect::<Vec<usize>>();
        let players = game_info[0];
        let num_marbles = match prob {
            ProblemPart::A => game_info[1],
            ProblemPart::B => game_info[1] * 100
        };

        let mut board = VecDeque::with_capacity(num_marbles);
        board.push_front(0);
        let mut score = vec![0usize; players];
        let mut cur_elf = 0;

        for marble in 1..=num_marbles {
            if marble % 23 == 0 {
                for _ in 0..7 {
                    let el = board.pop_back().unwrap();
                    board.push_front(el);
                }
                let rem = board.pop_front().unwrap();
                score[cur_elf] += marble + rem;
            } else {
                for _ in 0..=1 {
                    let el = board.pop_front().unwrap();
                    board.push_back(el);
                }
                board.push_front(marble);
            }

            cur_elf = (cur_elf + 1) % players;
        }
        score.iter().max().unwrap().to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day9_test1() {
        assert_eq!("32", Day9.solve_a("9 players; last marble is worth 25 points"));
        assert_eq!("8317", Day9.solve_a("10 players; last marble is worth 1618 points"));
        assert_eq!("2764", Day9.solve_a("17 players; last marble is worth 1104 points"));
        assert_eq!("54718", Day9.solve_a("21 players; last marble is worth 6111 points"));
        assert_eq!("37305", Day9.solve_a("30 players; last marble is worth 5807 points"));
        assert_eq!("146373", Day9.solve_a("13 players; last marble is worth 7999 points"));
        assert_eq!("370210", Day9.solve_a("464 players; last marble is worth 70918 points"));

        assert_eq!("22563", Day9.solve_b("9 players; last marble is worth 25 points"));
    }
}