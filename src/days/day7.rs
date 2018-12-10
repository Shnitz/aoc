use std::collections::HashMap;
use std::iter::FromIterator;

use petgraph::stable_graph::StableGraph;
use petgraph::graph::NodeIndex;
use petgraph::Direction;

use aoc::*;
use days::ChristmasDay;

pub struct Day7;

impl ChristmasDay for Day7 {
    fn solve(&self, data: &str, prob: ProblemPart) -> String {
        if prob == ProblemPart::B {
            return self.solve_for_time(data, 5, 60);
        }

        let mut steps = data.lines().map(|s| {
            let words = s.chars().collect::<Vec<char>>();
            (words[5], words[36])
        }).collect::<Vec<(char, char)>>();
        steps.sort_by_key(|e| e.1);

        // Find the root nodes.
        let tails = steps.iter().map(|e| e.1).collect::<Vec<char>>();
        let mut roots = steps.iter().filter_map(|e| {
            if !tails.contains(&e.0) { Some(e.0.clone()) } else { None }
        }).collect::<Vec<char>>();
        roots.sort();
        roots.dedup();

        let mut tree = StableGraph::<char, usize>::new();
        let root = tree.add_node('#');

        let mut node_ids = steps.iter().flat_map(|e| vec![e.0, e.1]).collect::<Vec<char>>();
        node_ids.sort();
        node_ids.dedup();
        let mut nodes = HashMap::new();
        node_ids.iter().for_each(|&node| { nodes.insert(node, tree.add_node(node)); });

        steps.iter().for_each(|(from, to)| {
            tree.add_edge(*nodes.get(from).unwrap(), *nodes.get(to).unwrap(), 0);
        });
        roots.iter().for_each(|r| { tree.add_edge(root, *nodes.get(r).unwrap(), 0); });

        let mut output = vec![];
        while tree.node_count() > 1 {
            let mut options = tree.neighbors(root).filter_map(|n| {
                let id = *tree.node_weight(n).unwrap();
                if tree.neighbors_directed(n, Direction::Incoming).any(|f| {
                    *tree.node_weight(f).unwrap() != '#'
                }) { None } else { Some((id, n)) }
            }).collect::<Vec<(char, NodeIndex)>>();
            options.sort_by_key(|e| e.0);
            let element = options.first().unwrap();
            // create new edges
            let edges = tree.neighbors(element.1).map(|n| *tree.node_weight(n).unwrap())
                .collect::<Vec<char>>();
            tree.remove_node(element.1);
            edges.iter().for_each(|c| { tree.add_edge(root, *nodes.get(c).unwrap(), 0); });
            output.push(element.0);
        }

        String::from_iter(output)
    }
}

impl Day7 {
    fn solve_for_time(&self, data: &str, num_workers: usize, default_time: usize) -> String {
        let mut steps = data.lines().map(|s| {
            let words = s.chars().collect::<Vec<char>>();
            (words[5], words[36])
        }).collect::<Vec<(char, char)>>();
        steps.sort_by_key(|e| e.1);

        // Find the root nodes.
        let tails = steps.iter().map(|e| e.1).collect::<Vec<char>>();
        let mut roots = steps.iter().filter_map(|e| {
            if !tails.contains(&e.0) { Some(e.0.clone()) } else { None }
        }).collect::<Vec<char>>();
        roots.sort();
        roots.dedup();

        let mut tree = StableGraph::<char, usize>::new();
        let mut nodes = HashMap::new();

        let root = tree.add_node('#');

        let mut node_ids = steps.iter().flat_map(|e| vec![e.0, e.1]).collect::<Vec<char>>();
        node_ids.sort();
        node_ids.dedup();
        node_ids.iter().for_each(|&node| { nodes.insert(node, tree.add_node(node)); });

        steps.iter().for_each(|(from, to)| {
            tree.add_edge(*nodes.get(from).unwrap(), *nodes.get(to).unwrap(), 0);
        });

        roots.iter().for_each(|r| { tree.add_edge(root, *nodes.get(r).unwrap(), 0); });

        let mut time = 0;
        let mut output = vec![];
        let mut workers: Vec<(usize, char)> = vec![(0usize, '#'); num_workers];
        while tree.node_count() > 1 {
            workers.iter_mut().for_each(|(time, task)| {
                if *time == 0  && *task != '#' {
                    let to_remove = *nodes.get(&task).unwrap();
                    let edges = tree.neighbors(to_remove).map(|n| *tree.node_weight(n).unwrap())
                        .collect::<Vec<char>>();
                    tree.remove_node(to_remove);
                    edges.iter().for_each(|c| { tree.add_edge(root, *nodes.get(c).unwrap(), 0); });
                    output.push(*task);
                    *task = '#';
                }
            });
            for wid in 0..workers.len() {
                if workers[wid].0 == 0 {
                    let mut options = tree.neighbors(root).filter_map(|n| {
                        let id = *tree.node_weight(n).unwrap();
                        if tree.neighbors_directed(n, Direction::Incoming).any(|f| {
                            *tree.node_weight(f).unwrap() != '#'
                        }) { None } else { Some((id, n)) }
                    })
                        .filter(|(key, _)| {
                            !workers.iter().any(|(_, id)| key == id)
                        }).collect::<Vec<(char, NodeIndex)>>();
                    options.sort_by_key(|e| e.0);
                    match options.first() {
                        Some(element) => {
                            // Calculate work time.
                            workers[wid].0 += default_time + (element.0.to_ascii_lowercase().to_digit(36).unwrap() as usize) - 10;
                            workers[wid].1 = element.0;
                        },
                        None => {}
                    }
                } else {
                    workers[wid].0 -= 1;
                }
            }
            time += 1;
        }
        let ret = String::from_iter(output);
        println!("CABFDE == {}", ret);

        (time + workers.iter().max_by_key(|e| e.0).unwrap().0 - 1).to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day7_test1() {
        assert_eq!("CABDFE", Day7.solve_a("Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin."));
        assert_eq!("15", Day7.solve_for_time("Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.", 2, 0));
    }
}