use aoc::*;
use days::ChristmasDay;

pub struct Day8;

#[derive(Debug, Copy, Clone)]
enum State {
    NewNode,
    ReadMeta,
}

#[derive(Debug)]
struct Node {
    id: usize,
    cn: usize,
    mn: usize,
    tot: usize,
    ch: Vec<usize>,
}

impl ChristmasDay for Day8 {
    fn solve(&self, data: &str, prob: ProblemPart) -> String {
        let mut atot = 0;
        let mut btot = 0;

        let mut nums = data.split_whitespace().map(|e| e.parse::<usize>().unwrap()).collect::<Vec<usize>>();

        let mut nodes = vec![];
        let mut state: State = State::NewNode;
        let mut idcnt = 1;

        while !nums.is_empty() {
            match state {
                State::NewNode => {
                    let node = Node { id: idcnt, cn: nums.remove(0), mn: nums.remove(0), tot: 0, ch: vec![] };
                    idcnt += 1;
                    if node.cn == 0 && node.mn != 0 {
                        state = State::ReadMeta;
                    }
                    nodes.push(node);
                }
                State::ReadMeta => {
                    match nodes.last_mut() {
                        Some(node) => {
                            let metadata = nums.drain(..node.mn);
                            match prob {
                                ProblemPart::A => {
                                    let sum: usize = metadata.sum();
                                    atot += sum;
                                }
                                ProblemPart::B => {
                                    if node.cn == 0 {
                                        let sum: usize = metadata.sum();
                                        node.tot = sum;
                                    } else {
                                        metadata.for_each(|md| {
                                            if md != 0 {
                                                match node.ch.get(md - 1) {
                                                    Some(child) => node.tot += child,
                                                    None => {}
                                                }
                                            }
                                        });
                                    }
                                }
                            }
                        }
                        None => {}
                    }
                    btot = nodes.pop().unwrap().tot;
                    state = match nodes.last_mut() {
                        Some(mut node) => {
                            node.ch.push(btot);
                            if node.cn == node.ch.len() { State::ReadMeta } else { State::NewNode }
                        }
                        None => State::NewNode
                    };
                }
            }
        }
        match prob {
            ProblemPart::A => atot.to_string(),
            ProblemPart::B => btot.to_string()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day8_test1() {
//        2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2
//        1----------------------------------
//            2----------- 3-----------
//                             4-----
        assert_eq!("138", Day8.solve_a("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"));
    }

    #[test]
    fn day8_test2() {
        assert_eq!("66", Day8.solve_b("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"));
    }
}