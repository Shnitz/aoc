use aoc::*;
use days::ChristmasDay;
use std::collections::HashMap;

pub struct Day7;

#[derive(Debug, Clone)]
struct Node {
    name: String,
    weight: i32,
    parent: Option<String>,
    children: Vec<String>,
    cweight: i32
}

impl ChristmasDay for Day7 {
    fn solve(&self, data: &str, prob: ProblemPart) -> String {
        let mut tree: HashMap<String, Node> = HashMap::new();

        data.lines().for_each(|line| {
            let nodes: Vec<String> = line.split_whitespace().map(String::from).collect::<Vec<String>>();
            let mut nodes_iter = nodes.iter();
            // "xhth (57)"
            // "fwft (72) -> ktlj, cntj, xhth"
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
                parent.cweight += children_weight;
            }
        });

        println!("Tree: --------------------");
        for n in &tree {
            println!("{:?}", n);
        }
        let root: &String = &tree.values().filter(|n| n.parent == None).last().unwrap().name;

        match prob {
            ProblemPart::A => root.to_string(),
            ProblemPart::B => {
                // let candidates: Vec<(String, i32)> = tree.iter().map(|n| (n.parent.unwrap(), n.weight)).collect();
                //
                // candidates.iter().for_each(|n| {
                //     let mut parent = tree.iter().find(|p| p.name == n.0).unwrap();
                //
                // });
                // println!("Err: {:?}", candidates);
                
                "".to_string()
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
        assert_eq!("8", Day7.solve_b(t));
    }
}
