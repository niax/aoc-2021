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

fn test_y(mut dy: isize, y_range: &RangeInclusive<isize>, min_y: isize) -> bool {
    let mut pos = 0;
    while pos >= min_y {
        if y_range.contains(&pos) {
            return true;
        }
        pos += dy;
        dy -= 1;
    }

    false
}

fn test_velocity(
    mut velocity: Vec2,
    x_range: &RangeInclusive<isize>,
    y_range: &RangeInclusive<isize>,
) -> Option<isize> {
    let mut pos = (0, 0);
    let mut max_height = isize::MIN;
    let min_y = *y_range.start().min(y_range.end());
    let max_x = *x_range.start().max(x_range.end());
    while pos.0 <= max_x && pos.1 >= min_y {
        max_height = max_height.max(pos.1);

        if x_range.contains(&pos.0) && y_range.contains(&pos.1) {
            return Some(max_height);
        }
        let (new_pos, new_velocity) = step(pos, velocity);
        pos = new_pos;
        velocity = new_velocity;
    }

    None
}

fn main() {
    let line = load_argv_lines::<String>().next().unwrap().unwrap();
    let caps = RE.captures(&line).unwrap();
    let x1 = caps.get(1).unwrap().as_str().parse::<isize>().unwrap();
    let x2 = caps.get(2).unwrap().as_str().parse::<isize>().unwrap();
    let y1 = caps.get(3).unwrap().as_str().parse::<isize>().unwrap();
    let y2 = caps.get(4).unwrap().as_str().parse::<isize>().unwrap();
    let x_range = x1..=x2;
    let y_range = y1..=y2;

    let min_y = *y_range.start().min(y_range.end());
    let y = y_range.end().abs();

    let valid_y: Vec<_> = (-y * 4..y * 4)
        .filter(|p| test_y(*p, &y_range, min_y))
        .collect();

    let mut max_height = isize::MIN;
    let mut possible_count = 0;
    for dx in 1..=x2 {
        for dy in &valid_y {
            if let Some(height) = test_velocity((dx, *dy), &x_range, &y_range) {
                max_height = std::cmp::max(max_height, height);
                possible_count += 1;
            }
        }
    }
    println!("{}", max_height);
    println!("{}", possible_count);
}
