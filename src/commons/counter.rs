use std::collections::HashMap;

#[derive(Debug)]
pub struct HashCounter<T> {
    m: HashMap<T, usize>,
}

impl<T> HashCounter<T>
where
    T: std::hash::Hash + std::cmp::Eq,
{
    pub fn new() -> Self {
        Self { m: HashMap::new() }
    }

    pub fn incr(&mut self, value: T) {
        let slot = self.m.entry(value).or_insert(0);
        *slot += 1;
    }

    pub fn get(&mut self, value: &T) -> Option<usize> {
        self.m.get(value).copied()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&T, &usize)> {
        self.m.iter()
    }

    pub fn len(&self) -> usize {
        self.m.len()
    }

    pub fn is_empty(&self) -> bool {
        self.m.is_empty()
    }
}

impl<T> Default for HashCounter<T>
where
    T: std::hash::Hash + std::cmp::Eq,
{
    fn default() -> Self {
        Self::new()
    }
}
