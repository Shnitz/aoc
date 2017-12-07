use aoc::*;
use days::ChristmasDay;

pub struct Day7;

#[derive(Debug, Clone)]
struct Node {
    name: String,
    weight: Option<i32>,
    parent: Option<String>,
    children: Vec<String>,
}

impl ChristmasDay for Day7 {
    fn solve(&self, data: &str, prob: ProblemPart) -> String {
        let mut tree: Vec<Node> = Vec::new();

        data.lines().for_each(|line| {
            let nodes: Vec<String> = line.split_whitespace().map(String::from).collect::<Vec<String>>();
            let mut nodes_iter = nodes.iter();
            // "xhth (57)"
            // "fwft (72) -> ktlj, cntj, xhth"
            let pname = nodes_iter.next().unwrap().to_string();
            let pweight = nodes_iter.next().unwrap();
            let weight = pweight.chars().skip(1).take(pweight.len() - 2).collect::<String>().parse::<i32>().unwrap();
            let tmp_node = Node {
                name: pname.clone(),
                weight: Some(weight),
                parent: None,
                children: Vec::new(),
            };
            match tree.iter().position(|node| node.name == pname) {
                Some(idx) => tree.iter().nth(idx).unwrap(),
                None => {
                    tree.push(tmp_node.clone());
                    tree.iter().last().unwrap()
                },
            };
            let mut children: Vec<String> = Vec::new();
            nodes_iter
                .skip(1)
                .for_each(|n| {
                    let name: String = n.trim_matches(',').to_string();
                    children.push(name.clone());
                    let tmp_node = Node {
                        name: name.clone(),
                        weight: Some(weight),
                        parent: None,
                        children: Vec::new(),
                    };
                    let node: &mut Node = match tree.iter().position(|node| node.name == name) {
                        Some(idx) => tree.iter_mut().nth(idx).unwrap(),
                        None => {
                            tree.push(tmp_node.clone());
                            tree.iter_mut().last().unwrap()
                        },
                    };
                    node.parent = Some(pname.clone());
                    println!("{:?}", node);
                });

            let parent: &mut Node = tree.iter_mut().find(|n| n.name == pname).unwrap();
            parent.children.extend(children);
        });

        let root: &String = &tree.iter().filter(|n| n.parent == None).last().unwrap().name;
        match prob {
            ProblemPart::A => root.to_string(),
            ProblemPart::B => {
                let candidates: Vec<&Node> = tree.iter().filter(|n| n.children.len() != 0).collect();
                println!("Err: {:?}", candidates);
                "".to_string()
            },
        }

    }
}

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
        assert_eq!("8", Day7.solve_b(t));
    }
}
