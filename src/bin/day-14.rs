use aoc2021::commons::io::load_stdin_lines;
use std::collections::HashMap;

fn answer(v: &HashMap<(char, char), u128>) {
    let mut counts = HashMap::new();
    for (pair, count) in v {
        *counts.entry(pair.0).or_insert(0) += count;
        *counts.entry(pair.1).or_insert(0) += count;
    }
    counts.remove(&'_');
    let min = counts.iter().map(|(_, c)| c).min().unwrap();
    let max = counts.iter().map(|(_, c)| c).max().unwrap();
    println!("{}", (max - min) / 2);
}

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
            let mut rule = a.chars();
            rules.insert(
                (rule.next().unwrap(), rule.next().unwrap()),
                b.chars().nth(0).unwrap(),
            );
        }
    }
    // Wrap the template in _ to handle the pairwise addition later
    template.insert(0, '_');
    template.push('_');

    let mut pair_counts = HashMap::<(char, char), u128>::new();
    for pair in template.windows(2) {
        *pair_counts.entry((pair[0], pair[1])).or_insert(0) += 1;
    }

    for i in 1..=40 {
        let mut next = HashMap::with_capacity(pair_counts.len());
        for (pair, count) in pair_counts {
            match rules.get(&pair) {
                Some(insert) => {
                    let first = next.entry((pair.0, *insert)).or_insert(0);
                    *first += count;
                    let second = next.entry((*insert, pair.1)).or_insert(0);
                    *second += count;
                }
                None => {
                    *next.entry(pair).or_insert(0) += 1;
                }
            }
        }
        pair_counts = next;
        if i == 10 {
            answer(&pair_counts);
        }
    }
    answer(&pair_counts);
}
