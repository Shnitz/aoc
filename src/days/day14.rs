use aoc::*;
use days::ChristmasDay;

pub struct Day14;

impl ChristmasDay for Day14 {
    fn solve(&self, data: &str, _prob: ProblemPart) -> String {
        let recipes = data.parse::<usize>().unwrap();

        let mut elfs = vec![0, 1];
        let mut scores = vec![3, 7];

        while scores.len() < recipes + 10 {
            let next_recipe = scores[elfs[0]] + scores[elfs[1]];

            let ones = next_recipe % 10;
            let tens = (next_recipe / 10) % 10;

            if tens > 0 { scores.push(tens); }
            scores.push(ones);

            let lim = scores.len();
            elfs[0] = (elfs[0] + scores[elfs[0]] + 1) % lim;
            elfs[1] = (elfs[1] + scores[elfs[1]] + 1) % lim;
        }

        scores[recipes..recipes + 10].to_vec().iter().map(|w| w.to_string()).collect::<Vec<String>>().join("")
    }

    fn solve_b(&self, data: &str) -> String {
        let recipes = data.chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<usize>>();
        let recipe_len = recipes.len();

        let mut elfs = vec![0, 1];
        let mut scores = vec![3, 7];

        while scores.len() < recipe_len || recipes[..] != scores[scores.len() - recipe_len..] {
            let next_recipe = scores[elfs[0]] + scores[elfs[1]];

            let ones = next_recipe % 10;
            let tens = (next_recipe / 10) % 10;

            if tens > 0 {
                scores.push(tens);

                if scores.len() > recipe_len && recipes[..] == scores[scores.len() - recipe_len..] {
                    break;
                }
            }
            scores.push(ones);

            let lim = scores.len();
            elfs[0] = (elfs[0] + scores[elfs[0]] + 1) % lim;
            elfs[1] = (elfs[1] + scores[elfs[1]] + 1) % lim;
        }

        (scores.len() - recipe_len).to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day24_test1() {
        assert_eq!("5158916779", Day14.solve_a("9"));
        assert_eq!("0124515891", Day14.solve_a("5"));
        assert_eq!("9251071085", Day14.solve_a("18"));
        assert_eq!("5941429882", Day14.solve_a("2018"));

        assert_eq!("9", Day14.solve_b("51589"));
        assert_eq!("5", Day14.solve_b("01245"));
        assert_eq!("18", Day14.solve_b("92510"));
        assert_eq!("2018", Day14.solve_b("59414"));
    }
}