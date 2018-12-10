use aoc::*;
use days::ChristmasDay;

pub struct Day9;

impl ChristmasDay for Day9 {
    fn solve(&self, data: &str, prob: ProblemPart) -> String {
        let game_info = data.split_whitespace().filter_map(|e| e.parse::<usize>().ok()).collect::<Vec<usize>>();
        let players = game_info[0];
        let num_marbles = match prob {
            ProblemPart::A => game_info[1],
            ProblemPart::B => game_info[1] * 100
        };

        let mut board = vec![0];
        let mut score = vec![0usize; players];
        let mut cur_marble = 0;
        let mut cur_elf = 0;

        for marble in 1..=num_marbles {
            if marble % 23 == 0 {
                cur_marble = (cur_marble + board.len() - 7) % board.len();
                let rem = board.remove(cur_marble);
                score[cur_elf] += marble + rem;

                println!("-> {} score !{}! {}", marble - 4, rem, rem + marble);
            } else {
                cur_marble = ((cur_marble + 1) % board.len()) + 1;
                board.insert(cur_marble, marble);
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
        assert_eq!("32", Day9.solve_a("9 players; last marble is worth 1000 points"));

//        assert_eq!("32", Day9.solve_a("9 players; last marble is worth 25 points"));
//        assert_eq!("8317", Day9.solve_a("10 players; last marble is worth 1618 points"));
//        assert_eq!("2764", Day9.solve_a("17 players; last marble is worth 1104 points"));
//        assert_eq!("54718", Day9.solve_a("21 players; last marble is worth 6111 points"));
//        assert_eq!("37305", Day9.solve_a("30 players; last marble is worth 5807 points"));
//        assert_eq!("146373", Day9.solve_a("13 players; last marble is worth 7999 points"));
//        assert_eq!("370210", Day9.solve_a("464 players; last marble is worth 70918 points"));

//        assert_eq!("370210", Day9.solve_b("464 players; last marble is worth 70918 points"));
    }
}