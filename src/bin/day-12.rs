use aoc2021::commons::io::load_stdin_lines;
use std::collections::{HashMap, HashSet};
use petgraph::graph::{NodeIndex, Graph, UnGraph};

#[derive(Debug,Hash, Clone, Eq, PartialEq)]
struct Cave {
    name: String,
    small: bool
}

impl Cave {
    pub fn parse(s: String) -> Self {
        let small = s.to_lowercase() == s;

        Self {
            name: s,
            small,
        }
    }
}


fn find_paths(graph: &UnGraph<Cave, ()>, current: NodeIndex<u32>, path: Vec<Cave>, visited_small: HashSet<Cave>) -> u32{
    let mut new_path = path.clone();
    new_path.push(graph.node_weight(current).unwrap().clone());
    let mut valid_paths = 0;
    if graph.node_weight(current).unwrap().name == "end" {
        let mut s = String::new();
        for n in new_path {
            s += &n.name;
            s.push(',');
        }
        //println!("{}", s);
        return 1;
    }
    for neigh in graph.neighbors(current) {
        let mut new_visited = visited_small.clone();
        let cave = graph.node_weight(neigh).unwrap();
        if cave.small {
            if visited_small.contains(cave) {
                continue;
            } 
            new_visited.insert(cave.clone());
        }
        valid_paths += find_paths(graph, neigh, new_path.clone(), new_visited);
    }
    return valid_paths;
}

fn main() {
    let mut graph = Graph::new_undirected();
    let mut node_ids = HashMap::new();
    for line in load_stdin_lines::<String>() {
        let line = line.unwrap();
        let (a, b) = line.split_once('-').unwrap();
        let a_node = node_ids.entry(a.to_string()).or_insert_with(
            || graph.add_node(Cave::parse(a.to_string()))
        ).clone();
        let b_node = node_ids.entry(b.to_string()).or_insert_with(
            || graph.add_node(Cave::parse(b.to_string()))
        ).clone();

        graph.add_edge(a_node, b_node, ());
    }

    let start = node_ids["start"];
    let mut visited = HashSet::new();
    visited.insert(graph.node_weight(start).unwrap().clone());
    let part1 = find_paths(&graph, start, Vec::new(), visited);
    println!("{}", part1);
}
