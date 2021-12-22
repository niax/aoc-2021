use aoc2021::commons::{counter::HashCounter, geom::Point3D, io::load_argv_lines};
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

type Point = Point3D<isize>;

fn rotations() -> Vec<&'static dyn Fn(&Point) -> Point> {
    vec![
        &|p| Point::new(*p.x(), *p.y(), *p.z()),
        &|p| Point::new(-*p.x(), -*p.y(), *p.z()),
        &|p| Point::new(-*p.x(), *p.y(), -*p.z()),
        &|p| Point::new(*p.x(), -*p.y(), -*p.z()),
        &|p| Point::new(*p.y(), *p.z(), *p.x()),
        &|p| Point::new(-*p.y(), -*p.z(), *p.x()),
        &|p| Point::new(-*p.y(), *p.z(), -*p.x()),
        &|p| Point::new(*p.y(), -*p.z(), -*p.x()),
        &|p| Point::new(*p.z(), *p.x(), *p.y()),
        &|p| Point::new(-*p.z(), -*p.x(), *p.y()),
        &|p| Point::new(-*p.z(), *p.x(), -*p.y()),
        &|p| Point::new(*p.z(), -*p.x(), -*p.y()),
        &|p| Point::new(*p.x(), *p.z(), -*p.y()),
        &|p| Point::new(*p.x(), -*p.z(), *p.y()),
        &|p| Point::new(-*p.x(), *p.z(), *p.y()),
        &|p| Point::new(-*p.x(), -*p.z(), -*p.y()),
        &|p| Point::new(*p.y(), *p.x(), -*p.z()),
        &|p| Point::new(*p.y(), -*p.x(), *p.z()),
        &|p| Point::new(-*p.y(), *p.x(), *p.z()),
        &|p| Point::new(-*p.y(), -*p.x(), -*p.z()),
        &|p| Point::new(*p.z(), *p.y(), -*p.x()),
        &|p| Point::new(*p.z(), -*p.y(), *p.x()),
        &|p| Point::new(-*p.z(), *p.y(), *p.x()),
        &|p| Point::new(-*p.z(), -*p.y(), -*p.x()),
    ]
}

#[derive(Clone, Debug)]
struct Scanner {
    index: usize,
    points: HashSet<Point>,
    axis_distance_pairs: HashMap<Point, (Point, Point)>,
    rotations: Vec<Scanner>,
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            index: usize::MAX,
            points: HashSet::new(),
            axis_distance_pairs: HashMap::new(),
            rotations: Vec::new(),
        }
    }

    pub fn rotations(&mut self) -> &mut Vec<Scanner> {
        if self.rotations.is_empty() {
            for rotation in rotations() {
                self.rotations.push(self.map_points(rotation));
            }
        }
        &mut self.rotations
    }

    fn axis_distances(&mut self) -> &HashMap<Point, (Point, Point)> {
        if self.axis_distance_pairs.is_empty() {
            for perm in self.points.iter().permutations(2) {
                let (a, b) = if perm[0].magnitude_squared() > perm[1].magnitude_squared() {
                    (*perm[0], *perm[1])
                } else {
                    (*perm[1], *perm[0])
                };
                let distances = b - a;
                let prev = self.axis_distance_pairs.insert(distances, (a, b));
                if prev.is_some() && !(prev.unwrap().0 == a && prev.unwrap().1 == b) {
                    panic!("Duplicate axis differences");
                }
            }
        }
        &self.axis_distance_pairs
    }

    fn map_points<F>(&self, f: F) -> Self
    where
        F: FnMut(&Point) -> Point,
    {
        let points = self.points.iter().map(f).collect();
        Self {
            index: self.index,
            points,
            axis_distance_pairs: HashMap::new(),
            rotations: Vec::new(),
        }
    }

    pub fn merge(&mut self, other: &mut Scanner) -> Option<Point> {
        let known_distances = self.axis_distances();
        let mut best_counters = HashMap::new();
        let mut best_points = HashSet::new();
        for rotated in other.rotations() {
            let mut counters = HashMap::new();
            let rotated_distances = rotated.axis_distances();
            for (distance, other_points) in rotated_distances {
                if let Some(our_points) = known_distances.get(distance) {
                    for p in [other_points.0, other_points.1] {
                        let counter = counters.entry(p).or_insert_with(HashCounter::new);
                        for p2 in [our_points.0, our_points.1] {
                            counter.incr(p2);
                        }
                    }
                }
            }
            if counters.len() > best_counters.len() {
                best_counters = counters;
                best_points = rotated.points.clone();
            }
        }

        if best_counters.len() >= 5 {
            let mut translations = HashSet::new();
            for (other, counter) in best_counters {
                let (ours, _) = counter.iter().max_by_key(|(_, c)| *c).unwrap();
                translations.insert(*ours - other);
            }

            if translations.len() == 1 {
                let translation = translations.iter().next().unwrap();
                for p in &best_points {
                    let translated = *p + *translation;
                    self.points.insert(translated);
                }
                self.axis_distance_pairs.clear();
                return Some(*translation);
            }
        }
        None
    }
}

fn main() {
    let lines = load_argv_lines::<String>().map(|l| l.unwrap());
    let mut scanners = VecDeque::new();
    let mut scanner = Scanner::new();
    for line in lines {
        if line.starts_with("---") {
            if scanner.index != usize::MAX {
                scanners.push_back(scanner);
            }
            scanner = Scanner::new();
            scanner.index = line.split_whitespace().nth(2).unwrap().parse().unwrap();
        } else if line.is_empty() {
            // Skip!
        } else {
            let mut parts = line.split(',').map(|x| x.parse().unwrap());
            let x = parts.next().unwrap();
            let y = parts.next().unwrap();
            let z = parts.next().unwrap();

            scanner.points.insert(Point3D::new(x, y, z));
        }
    }
    scanners.push_back(scanner);

    let mut scanner0 = scanners.pop_front().unwrap();
    let mut distances = vec![Point::origin()];

    while !scanners.is_empty() {
        let mut scanner = scanners.pop_front().unwrap();
        match scanner0.merge(&mut scanner) {
            Some(translation) => distances.push(translation),
            None => {
                scanners.push_back(scanner);
            }
        }
    }
    println!("{}", scanner0.points.len());

    let max = distances
        .iter()
        .permutations(2)
        .map(|perm| perm[0].manhattan_distance(perm[1]))
        .max()
        .unwrap();
    println!("{}", max);
}
