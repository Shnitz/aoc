extern crate petgraph;

use aoc::*;
use days::ChristmasDay;
use self::petgraph::visit::{ Bfs, Walker };
use std::collections::HashMap;

pub struct Day12;

impl ChristmasDay for Day12 {
    fn solve(&self, data: &str, prob: ProblemPart) -> String {
        let mut graph = petgraph::Graph::<&str, &str, petgraph::Undirected>::default();
        let mut nodes: HashMap<&str, petgraph::graph::NodeIndex> = HashMap::new();
        nodes.insert("0", graph.add_node("0"));
        data.lines().for_each(|line| {
            let mut params = line.split_whitespace();
            let id = params.next().unwrap();
            nodes.entry(id).or_insert_with(|| graph.add_node(id));
            params.skip(1).for_each(|n| {
                let tid = n.trim_matches(',');
                nodes.entry(tid).or_insert_with(|| graph.add_node(tid));
                graph.update_edge(*nodes.get(id).unwrap(), *nodes.get(tid).unwrap(), "");
            });
        });

        match prob {
            ProblemPart::A => Bfs::new(&graph, *nodes.get("0").unwrap()).iter(&graph).count().to_string(),
            ProblemPart::B => petgraph::algo::connected_components(&graph).to_string()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day12_test1() {
        let test_str = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";
        assert_eq!("6", Day12.solve_a(test_str));
        assert_eq!("2", Day12.solve_b(test_str));
    }
}
