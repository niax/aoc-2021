use aoc2021::commons::io::load_argv_lines;
use bitvec::prelude::*;
use cached::{cached_key, SizedCache};
use petgraph::graph::{Graph, NodeIndex, UnGraph};
use std::collections::HashMap;

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct Cave {
    name: String,
    small: bool,
    start: bool,
    end: bool,
}

impl Cave {
    pub fn parse(s: String) -> Self {
        let small = s.to_lowercase() == s;
        let start = s == "start";
        let end = s == "end";

        Self {
            name: s,
            small,
            start,
            end,
        }
    }
}

cached_key! {
    PATHS: SizedCache<(usize, BitVec, bool), u32> = SizedCache::with_size(1_000_000);
    Key = { (current.index(), visited_small.clone(), visited_twice) };
    fn find_paths(graph: &UnGraph<Cave, ()>,
        current: NodeIndex<u32>,
        visited_small: &BitVec,
        visited_twice: bool
    ) -> u32  = {
        find_paths_actual(graph, current, visited_small, visited_twice)
    }
}

fn find_paths_actual(
    graph: &UnGraph<Cave, ()>,
    current: NodeIndex<u32>,
    visited_small: &BitVec,
    visited_twice: bool,
) -> u32 {
    if graph.node_weight(current).unwrap().end {
        1
    } else {
        graph
            .neighbors(current)
            .map(|neigh| {
                let mut inner_twice = visited_twice;
                let cave = graph.node_weight(neigh).unwrap();
                if cave.small {
                    if visited_small[neigh.index()] {
                        if inner_twice || cave.start {
                            return 0;
                        }
                        inner_twice = true;
                    }
                    let mut visited_clone = visited_small.clone();
                    *visited_clone.get_mut(neigh.index()).unwrap() = true;
                    find_paths(graph, neigh, &visited_clone, inner_twice)
                } else {
                    find_paths(graph, neigh, visited_small, inner_twice)
                }
            })
            .sum()
    }
}

fn main() {
    let mut graph = Graph::new_undirected();
    let mut node_ids = HashMap::new();
    for line in load_argv_lines::<String>() {
        let line = line.unwrap();
        let (a, b) = line.split_once('-').unwrap();
        let a_node = *node_ids
            .entry(a.to_string())
            .or_insert_with(|| graph.add_node(Cave::parse(a.to_string())));
        let b_node = *node_ids
            .entry(b.to_string())
            .or_insert_with(|| graph.add_node(Cave::parse(b.to_string())));

        graph.add_edge(a_node, b_node, ());
    }

    let start = node_ids["start"];
    let mut visited = bitvec![0; node_ids.len()];
    *visited.get_mut(start.index()).unwrap() = true;
    let part1 = find_paths(&graph, start, &visited, true);
    println!("{}", part1);

    let part2 = find_paths(&graph, start, &visited, false);
    println!("{}", part2);
}
