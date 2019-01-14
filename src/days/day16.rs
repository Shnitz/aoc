use aoc::*;

use days::ChristmasDay;

use std::collections::{HashMap, HashSet};

pub struct Day16;

#[derive(Debug, Copy, Clone)]
struct Sample {
    before: [i32; 4],
    op: [i32; 4],
    after: [i32; 4],
}

const PO_A: usize = 1;
const PO_B: usize = 2;
const PO_C: usize = 3;

impl ChristmasDay for Day16 {
    fn solve(&self, data: &str, prob: ProblemPart) -> String {
        let mut samples = vec![];

        let mut lines = data.lines();
        let mut empty = true;

        let mut next_sample: Sample = Sample { before: [0; 4], op: [0; 4], after: [0; 4] };
        while let Some(line) = lines.next() {
            let parts: Vec<&str> = line.split(|c| c == ' ' || c == '[' || c == ',' || c == ']').collect();
            let first: Vec<char> = line.chars().collect();

            if line.len() == 0 {
                if empty { break; } else { empty = true; }
                continue;
            }
            empty = false;

            if first[0] == 'B' {
                next_sample.before = [parts[2].parse::<i32>().unwrap(), parts[4].parse::<i32>().unwrap(), parts[6].parse::<i32>().unwrap(), parts[8].parse::<i32>().unwrap()];
            } else if first[0] == 'A' {
                next_sample.after = [parts[3].parse::<i32>().unwrap(), parts[5].parse::<i32>().unwrap(), parts[7].parse::<i32>().unwrap(), parts[9].parse::<i32>().unwrap()];
                samples.push(next_sample);
                next_sample = Sample { before: [0; 4], op: [0; 4], after: [0; 4] };
            } else {
                next_sample.op = [parts[0].parse::<i32>().unwrap(), parts[1].parse::<i32>().unwrap(), parts[2].parse::<i32>().unwrap(), parts[3].parse::<i32>().unwrap()]
            }
        }

        //Skip blank line.
        lines.next();
        let program = lines.map(|l| {
            let row = l.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            [row[0], row[1], row[2], row[3]]
        }).collect::<Vec<[i32; 4]>>();

        let same = |l: [i32; 4], r: [i32; 4]| -> bool {
            l[0] == r[0] && l[1] == r[1] && l[2] == r[2] && l[3] == r[3]
        };

        let ops: Vec<(&str, Box<Fn(&mut [i32; 4], [i32; 4])>)> = vec![
            ("addr", Box::new(|reg: &mut [i32; 4], op: [i32; 4]| { reg[op[PO_C] as usize] = reg[op[PO_A] as usize] + reg[op[PO_B] as usize]; })),
            ("addi", Box::new(|reg: &mut [i32; 4], op: [i32; 4]| { reg[op[PO_C] as usize] = reg[op[PO_A] as usize] + op[PO_B]; })),
            ("mulr", Box::new(|reg: &mut [i32; 4], op: [i32; 4]| { reg[op[PO_C] as usize] = reg[op[PO_A] as usize] * reg[op[PO_B] as usize]; })),
            ("muli", Box::new(|reg: &mut [i32; 4], op: [i32; 4]| { reg[op[PO_C] as usize] = reg[op[PO_A] as usize] * op[PO_B]; })),
            ("banr", Box::new(|reg: &mut [i32; 4], op: [i32; 4]| { reg[op[PO_C] as usize] = reg[op[PO_A] as usize] & reg[op[PO_B] as usize]; })),
            ("bani", Box::new(|reg: &mut [i32; 4], op: [i32; 4]| { reg[op[PO_C] as usize] = reg[op[PO_A] as usize] & op[PO_B]; })),
            ("borr", Box::new(|reg: &mut [i32; 4], op: [i32; 4]| { reg[op[PO_C] as usize] = reg[op[PO_A] as usize] | reg[op[PO_B] as usize]; })),
            ("bori", Box::new(|reg: &mut [i32; 4], op: [i32; 4]| { reg[op[PO_C] as usize] = reg[op[PO_A] as usize] | op[PO_B]; })),
            ("setr", Box::new(|reg: &mut [i32; 4], op: [i32; 4]| { reg[op[PO_C] as usize] = reg[op[PO_A] as usize]; })),
            ("seti", Box::new(|reg: &mut [i32; 4], op: [i32; 4]| { reg[op[PO_C] as usize] = op[PO_A]; })),
            ("gtir", Box::new(|reg: &mut [i32; 4], op: [i32; 4]| { reg[op[PO_C] as usize] = if op[PO_A] > reg[op[PO_B] as usize] { 1 } else { 0 }; })),
            ("gtri", Box::new(|reg: &mut [i32; 4], op: [i32; 4]| { reg[op[PO_C] as usize] = if reg[op[PO_A] as usize] > op[PO_B] { 1 } else { 0 }; })),
            ("gtrr", Box::new(|reg: &mut [i32; 4], op: [i32; 4]| { reg[op[PO_C] as usize] = if reg[op[PO_A] as usize] > reg[op[PO_B] as usize] { 1 } else { 0 }; })),
            ("eqir", Box::new(|reg: &mut [i32; 4], op: [i32; 4]| { reg[op[PO_C] as usize] = if op[PO_A] == reg[op[PO_B] as usize] { 1 } else { 0 }; })),
            ("eqri", Box::new(|reg: &mut [i32; 4], op: [i32; 4]| { reg[op[PO_C] as usize] = if reg[op[PO_A] as usize] == op[PO_B] { 1 } else { 0 }; })),
            ("eqrr", Box::new(|reg: &mut [i32; 4], op: [i32; 4]| { reg[op[PO_C] as usize] = if reg[op[PO_A] as usize] == reg[op[PO_B] as usize] { 1 } else { 0 }; })),
        ];

        let mut code_map: HashMap<i32, HashSet<&str>> = HashMap::new();

        // Count samples with multiple possibilities.
        let valid = samples.iter().filter(|&&sample| {
            let op_code: i32 = sample.op[0];
            let possibilities = ops.iter().filter(|op| {
                let mut reg = sample.before;
                op.1(&mut reg, sample.op);
                same(reg, sample.after)
            }).map(|op| op.0).collect::<HashSet<&str>>();

            let poss_len = possibilities.len();

            if !code_map.contains_key(&op_code) {
                code_map.insert(op_code, possibilities.clone());
            } else {
                let valid = code_map.get_mut(&op_code).unwrap();
                &mut valid.intersection(&possibilities).map(|&e| e).collect::<HashSet<&str>>();
            }

            poss_len >= 3
        }).count();

        if prob == ProblemPart::A {
            return valid.to_string();
        }

        let mut num_found = 0;
        while num_found != code_map.iter().filter(|(_, ops)| ops.len() == 1).count() {
            let mut known_l = code_map.clone();
            let known = known_l.iter_mut()
                .filter(|(_, ops)| ops.len() == 1)
                .map(|(id, op)| (id, op.iter().next().unwrap()))
                .collect::<Vec<(&i32, &&str)>>();
            num_found = known.len();
            for (id, ops) in known.iter().clone() {
                code_map.iter_mut().for_each(|(op, pot)| {
                    if op != *id {
                        pot.remove(*ops);
                    }
                })
            }
        }

        let mut codes = HashMap::new();
        code_map.iter().for_each(|(op, id)| {
            let code = *id.iter().next().unwrap();
            let func = &ops.iter().find(|e| e.0 == code).unwrap().1;
            codes.insert(*op, func);
        });

        // Run program
        let mut reg = [0i32; 4];
        program.iter().for_each(|row| {
            let func = codes.get(&row[0]).unwrap();
            func(&mut reg, *row);
        });

        reg[0].to_string()
    }
}

impl Day16 {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day16_test1() {
        assert_eq!("1", Day16.solve_a("Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]


9 2 1 2"));
    }

    #[test]
    fn day16_test2() {
        assert_eq!("0", Day16.solve_b("Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]


9 2 1 2"));
    }
}