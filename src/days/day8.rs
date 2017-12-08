use aoc::*;
use days::ChristmasDay;
use std::collections::HashMap;

pub struct Day8;

impl ChristmasDay for Day8 {
    fn solve(&self, data: &str, prob: ProblemPart) -> String {
        let mut registers: HashMap<&str, i32> = HashMap::new();
        let mut max_val = 0;
        data.lines().for_each(|line| {
            // 0 1   2 3  4 5 6
            // b inc 5 if a > 1
            let input = line.split_whitespace().collect::<Vec<&str>>();
            if {
                let chk_register_val: i32 = *registers.entry(input[4]).or_insert(0);
                let chk_amount: i32 = input[6].parse::<i32>().unwrap();
                match input[5] {
                    ">" => chk_register_val > chk_amount,
                    ">=" => chk_register_val >= chk_amount,
                    "<" => chk_register_val < chk_amount,
                    "<=" => chk_register_val <= chk_amount,
                    "==" => chk_register_val == chk_amount,
                    "!=" => chk_register_val != chk_amount,
                    _ => false,
                }
            } {
                let register = registers.entry(input[0]).or_insert(0);
                let amount: i32 = input[2].parse::<i32>().unwrap();
                match input[1] {
                    "inc" => { *register += amount; },
                    "dec" => { *register -= amount; },
                    _ => {},
                }
                if *register > max_val {
                    max_val = *register;
                }
            }
        });

        match prob {
            ProblemPart::A => registers.values().max().unwrap().to_string(),
            ProblemPart::B => max_val.to_string(),
        }

    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day8_test1() {
        let test = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";
        assert_eq!("1", Day8.solve_a(test));
        assert_eq!("10", Day8.solve_b(test));
    }
}
