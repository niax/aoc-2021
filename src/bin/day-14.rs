use aoc2021::commons::io::load_stdin_lines;
use std::collections::HashMap;

fn main() {
    let mut template = Vec::new();
    let mut rules = HashMap::new();
    for line in load_stdin_lines::<String>() {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }
        if template.is_empty() {
            template = line.chars().collect();
        } else {
            let (a, b) = line.split_once(" -> ").unwrap();
            rules.insert(a.to_string(), b.chars().nth(0).unwrap());
        }
    }

    for _ in 0..10 {
        let mut next = Vec::new();
        next.push(template[0]);
        for pair in template.windows(2) {
            let pair_str = pair.iter().collect::<String>();
            next.push(rules[&pair_str]);
            next.push(pair[1]);
        }
        template = next;
    }
    let mut counts = HashMap::new();
    for c in template {
        let counter = counts.entry(c).or_insert(0);
        *counter += 1;
    }
    let min = counts.iter().map(|(_, c)| c).min().unwrap();
    let max = counts.iter().map(|(_, c)| c).max().unwrap();
    println!("{}", max - min);
}
