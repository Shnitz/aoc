use aoc::*;
use days::ChristmasDay;
use std::collections::{HashMap, VecDeque};

pub struct Day7;

#[derive(Debug, Clone)]
struct Node {
    name: String,
    weight: i32,
    parent: Option<String>,
    children: Vec<String>,
    cweight: i32
}
impl Node {
    fn fullweight(&self) -> i32 {
        self.weight + (self.cweight * self.children.len() as i32)
    }
}

impl ChristmasDay for Day7 {
    fn solve(&self, data: &str, prob: ProblemPart) -> String {
        let mut tree: HashMap<String, Node> = HashMap::new();

        data.lines().for_each(|line| {
            let nodes: Vec<String> = line.split_whitespace().map(String::from).collect::<Vec<String>>();
            let mut nodes_iter = nodes.iter();
            let pname = nodes_iter.next().unwrap().to_string();
            let pweight = nodes_iter.next().unwrap();
            let weight = pweight.chars().skip(1).take(pweight.len() - 2).collect::<String>().parse::<i32>().unwrap();
            tree.entry(pname.clone()).or_insert(Node {
                name: pname.clone(),
                weight: weight,
                parent: None,
                children: Vec::new(),
                cweight: 0
            }).weight = weight;

            let mut children: Vec<String> = Vec::new();
            let mut children_weight: i32 = 0;
            nodes_iter
                .skip(1)
                .for_each(|n| {
                    let name: String = n.trim_matches(',').to_string();
                    children.push(name.clone());
                    let node = tree.entry(name.clone()).or_insert(Node {
                        name: name.clone(),
                        weight: 0,
                        parent: None,
                        children: Vec::new(),
                        cweight: 0
                    });
                    node.parent = Some(pname.clone());
                    children_weight += node.weight;
                });

            if let Some(parent) = tree.get_mut(&pname) {
                parent.children.extend(children);
            }
        });

        match prob {
            ProblemPart::A => tree.values().filter(|n| n.parent == None).last().unwrap().name.to_string(),
            ProblemPart::B => {
                let mut queue: VecDeque<String> = VecDeque::new();
                let mut err_node: Option<String> = None;
                tree.iter().filter(|n| n.1.children.len() == 0).for_each(|n| {
                    queue.push_back(n.0.clone());
                });

                while !queue.is_empty() {
                    let mut parentid: Option<String> = None;
                    let mut weight: Option<i32> = None;
                    if let Some(node) = tree.get_mut(&queue.pop_front().unwrap()) {
                        // Queue the parent
                        if let Some(parent) = node.parent.clone() {
                            if !queue.contains(&parent) {
                                queue.push_back(parent.clone());
                            }
                        }
                        parentid = node.parent.clone();
                        weight = Some(node.fullweight());
                    }
                    if let Some(node) = tree.get_mut(&parentid.unwrap()) {
                        if node.cweight == 0 {
                            node.cweight = weight.unwrap();
                        } else if node.cweight != weight.unwrap(){
                            queue.clear();
                            err_node = Some(node.name.clone());
                        }
                    }
                }

                let node = tree.get(&err_node.unwrap()).unwrap();
                let mut grouped: HashMap<i32, i32> = HashMap::new();
                let nodes = node.children.iter().map(|n| tree.get(n).unwrap()).collect::<Vec<&Node>>();
                nodes.iter().for_each(|child| {
                        let e = grouped.entry(child.fullweight()).or_insert(0);
                        *e += 1;
                    });
                let wrong = grouped.iter().min_by(|x, y| x.1.cmp(y.1)).unwrap().0;
                let right = grouped.iter().max_by(|x, y| x.1.cmp(y.1)).unwrap().0;
                let wrong_node = nodes.iter().find(|child| child.fullweight() == *wrong).unwrap();
                (wrong_node.weight - (wrong - right)).to_string()
            },
        }

    }
}
// ugml + (gyxo + ebii + jptl) = 68 + (61 + 61 + 61) = 251
// padx + (pbga + havc + qoyq) = 45 + (66 + 66 + 66) = 243
// fwft + (ktlj + cntj + xhth) = 72 + (57 + 57 + 57) = 243
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day7_test1() {
        let t = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";
        assert_eq!("tknk", Day7.solve_a(t));
        assert_eq!("60", Day7.solve_b(t));
    }
}
