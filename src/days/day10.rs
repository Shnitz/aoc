use aoc::*;
use days::ChristmasDay;

pub struct Day10;

impl ChristmasDay for Day10 {
    fn solve(&self, data: &str, prob: ProblemPart) -> String {
        let lengths: Vec<usize> = match prob {
            ProblemPart::A => data.split(',').map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>(),
            ProblemPart::B => {
                let mut tmp = data.as_bytes().iter().map(|&n| n as usize).collect::<Vec<usize>>();
                tmp.extend(vec![17, 31, 73, 47, 23]);
                tmp
            },
        };
        twist(256, lengths, prob)
    }
}

fn twist(length: usize, lengths: Vec<usize>, prob: ProblemPart) -> String {
    assert!(length > 1);
    let mut pos = 0;
    let mut skip = 0;
    let mut list: Vec<usize> = vec!(0; length);
    for x in 0..length { list[x as usize] = x; }
    for _ in 0.. match prob {
        ProblemPart::A => 1,
        ProblemPart::B => 64,
    } {
        for num in &lengths {
            let old : Vec<usize> = list.clone();
            let tmp = match pos + num {
                x if x >= list.len() => {
                    let mut tmp = old[pos..length].to_vec().clone();
                    tmp.extend(old[0..(x % length)].to_vec().clone());
                    tmp
                },
                x => old[pos..x].to_vec().clone(),
            };
            let mut slice = tmp.iter().rev().cycle();
            for idx in 0..*num {
                list[(pos + idx) % length] = slice.next().unwrap().clone();
            }
            pos = (pos + num + skip) % length;
            skip += 1;
        }
    }

    match prob {
        ProblemPart::A => (list[0] * list[1]).to_string(),
        ProblemPart::B => {
            list.chunks(16)
                .map(|w| format!("{:01$x}", w.iter().fold(0, |acc, &x| acc ^ x), 2))
                .collect::<Vec<String>>()
                .join("")
        },
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day10_test1() {
        assert_eq!("12", twist(5, vec![3, 4, 1, 5], ProblemPart::A));
        assert_eq!("a2582a3a0e66e6e86e3812dcb672a272", Day10.solve_b(""));
        assert_eq!("33efeb34ea91902bb2f59c9920caa6cd", Day10.solve_b("AoC 2017"));
        assert_eq!("3efbe78a8d82f29979031a4aa0b16a9d", Day10.solve_b("1,2,3"));
        assert_eq!("63960835bcdc130f0b66d7ff4f6a5a8e", Day10.solve_b("1,2,4"));
    }
}
