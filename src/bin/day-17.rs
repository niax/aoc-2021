use aoc2021::commons::io::load_argv_lines;
use lazy_static::lazy_static;
use regex::Regex;
use std::ops::RangeInclusive;

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)").unwrap();
}

type Vec2 = (isize, isize);

fn step(pos: Vec2, velocity: Vec2) -> (Vec2, Vec2) {
    let new_pos = (pos.0 + velocity.0, pos.1 + velocity.1);

    let new_dx = if velocity.0 == 0 { 0 } else { velocity.0 - 1 };
    let new_velocity = (new_dx, velocity.1 - 1);

    (new_pos, new_velocity)
}

#[allow(dead_code)]
struct Finder {
    x_range: RangeInclusive<isize>,
    y_range: RangeInclusive<isize>,
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
}

impl Finder {
    pub fn new(x1: isize, x2: isize, y1: isize, y2: isize) -> Self {
        let min_x = x1.min(x2);
        let max_x = x1.max(x2);
        let min_y = y1.min(y2);
        let max_y = y1.max(y2);
        let x_range = min_x..=max_x;
        let y_range = min_y..=max_y;
        Self {
            x_range,
            y_range,
            min_x,
            max_x, 
            min_y,
            max_y,
        }
    }

    pub fn test_x(&self, mut dx: isize) -> bool {
        let mut pos = 0;
        while pos <= self.max_x && dx > 0{
            if self.x_range.contains(&pos) {
                return true;
            }
            pos += dx;
            dx = if dx == 0 { 0 } else { dx - 1 };
        }

        false
    }

    pub fn test_y(&self, mut dy: isize) -> bool {
        let mut pos = 0;
        while pos >= self.min_y {
            if self.y_range.contains(&pos) {
                return true;
            }
            pos += dy;
            dy -= 1;
        }

        false
    }

    fn test_velocity(&self, mut velocity: Vec2) -> Option<isize> {
        let mut pos = (0, 0);
        let mut max_height = isize::MIN;
        while pos.0 <= self.max_x && pos.1 >= self.min_y {
            max_height = max_height.max(pos.1);

            if self.x_range.contains(&pos.0) && self.y_range.contains(&pos.1) {
                return Some(max_height);
            }
            let (new_pos, new_velocity) = step(pos, velocity);
            pos = new_pos;
            velocity = new_velocity;
        }

        None
    }
}

fn main() {
    let line = load_argv_lines::<String>().next().unwrap().unwrap();
    let caps = RE.captures(&line).unwrap();
    let x1 = caps.get(1).unwrap().as_str().parse::<isize>().unwrap();
    let x2 = caps.get(2).unwrap().as_str().parse::<isize>().unwrap();
    let y1 = caps.get(3).unwrap().as_str().parse::<isize>().unwrap();
    let y2 = caps.get(4).unwrap().as_str().parse::<isize>().unwrap();

    let finder = Finder::new(x1, x2, y1, y2);

    let y = finder.max_y.abs()*2;
    let valid_y: Vec<_> = (-y..=y)
        .filter(|p| finder.test_y(*p))
        .collect();

    let mut max_height = isize::MIN;
    let mut possible_count = 0;
    for dx in (1..=x2).filter(|x| finder.test_x(*x)) {
        for dy in &valid_y {
            if let Some(height) = finder.test_velocity((dx, *dy)) {
                max_height = std::cmp::max(max_height, height);
                possible_count += 1;
            }
        }
    }
    println!("{}", max_height);
    println!("{}", possible_count);
}
