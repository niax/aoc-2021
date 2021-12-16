use aoc2021::commons::io::load_argv_lines;
use std::collections::HashMap;

fn minmax<T, I>(iter: I) -> Option<(T, T)>
where
    T: std::cmp::Ord + Copy,
    I: std::iter::IntoIterator<Item = T>,
{
    let mut min = None;
    let mut max = None;

    for i in iter {
        if min.is_none() {
            min = Some(i);
            max = Some(i);
        }

        min = Some(std::cmp::min(min.unwrap(), i));
        max = Some(std::cmp::max(max.unwrap(), i));
    }

    min.map(|m| (m, max.unwrap()))
}

#[derive(Debug)]
struct HashCounter<T> {
    inner: HashMap<T, u128>,
}

impl<T> HashCounter<T>
where
    T: Eq + std::hash::Hash,
{
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn count(&mut self, value: T) {
        self.countn(value, 1);
    }

    pub fn countn(&mut self, value: T, n: u128) {
        *self.inner.entry(value).or_insert(0) += n;
    }

    pub fn iter(&self) -> impl Iterator<Item = (&T, &u128)> {
        self.inner.iter()
    }
}

fn answer(counter: &HashCounter<char>) {
    let (min, max) = minmax(counter.iter().map(|(_, c)| c)).unwrap();
    println!("{}", (max - min));
}

fn main() {
    let mut template = Vec::new();
    let mut rules = HashMap::new();
    for line in load_argv_lines::<String>() {
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
                b.chars().next().unwrap(),
            );
        }
    }

    let mut char_counts = HashCounter::new();
    for c in &template {
        char_counts.count(*c);
    }

    let mut pair_counts = HashMap::<(char, char), u128>::new();
    for pair in template.windows(2) {
        *pair_counts.entry((pair[0], pair[1])).or_insert(0) += 1;
    }

    for i in 1..=40 {
        let mut next = HashMap::with_capacity(pair_counts.len());
        for (pair, count) in pair_counts {
            match rules.get(&pair) {
                Some(insert) => {
                    char_counts.countn(*insert, count);
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
            answer(&char_counts);
        }
    }
    answer(&char_counts);
}
