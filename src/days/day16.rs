use aoc::*;
use days::ChristmasDay;

pub struct Day16;

impl ChristmasDay for Day16 {
    fn solve(&self, data: &str, prob: ProblemPart) -> String {
        let mut dancers = Vec::new();
        for i in 0..16 { dancers.push(('a' as u8 + i) as char); }
        let cmds = parse(data);
        match prob {
            ProblemPart::A => dance(dancers, cmds).into_iter().collect(),
            ProblemPart::B => {
                let mut ret = "".to_string();
                let mut all: Vec<String> = Vec::new();

                for x in 0..1_000_000_000 {
                    dancers = dance(dancers, cmds.clone());
                    let w = dancers.iter().clone().collect();
                    match all.iter().position(|d| d == &w) {
                        Some(p) => {
                            all.push(w);
                            let loos = (1_000_000_000 - x - 1) % (x - p);
                            ret = all[p + loos].clone();
                            break;
                        },
                        None => { all.push(w); }
                    }
                }
                ret
            },
        }
    }
}
fn parse(data: &str) -> Vec<(char, usize, usize)> {
    let mut cmds: Vec<(char, usize, usize)> = Vec::new();
    for cmd in data.split(',') {
        let mut it = cmd.chars();
        cmds.push(match it.next().unwrap() {
            's' => ('s',it.collect::<String>().parse::<usize>().unwrap(),0),
            'x' => {
                let m = it.collect::<String>();
                let mut n = m.split('/');
                ('x',n.next().unwrap().parse().unwrap(), n.next().unwrap().parse().unwrap())
            },
            'p' => ('p', it.next().unwrap() as usize, it.skip(1).next().unwrap() as usize),
            _ => (' ',0,0),
        })
    }
    cmds
}

fn dance(mut dcr: Vec<char>, cmds: Vec<(char, usize, usize)>) -> Vec<char> {
    let sz = dcr.len();

    for cmd in &cmds {
        match cmd.0 {
            's' => {
                let s = dcr.clone();
                for i in 0..sz {
                    dcr[(i + cmd.1) % sz] = s[i];
                }
            },
            'x' => { dcr.swap(cmd.1, cmd.2); },
            'p' => {
                let left = dcr.iter().position(|&c| c as usize == cmd.1).unwrap();
                let right = dcr.iter().position(|&c| c as usize == cmd.2).unwrap();
                dcr.swap(left, right);
            },
            _ => {},
        }
    }

    dcr
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day16_test1() {
        assert_eq!("baedc", dance(vec!('a','b','c','d','e'), parse("s1,x3/4,pe/b")).into_iter().collect::<String>());
    }
}
