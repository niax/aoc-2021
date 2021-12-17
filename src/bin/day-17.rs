use aoc2021::commons::io::load_argv_lines;
use lazy_static::lazy_static;
use regex::Regex;
use std::ops::RangeInclusive;

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"target area: x=(-?\d+)..(-?\d+), y=-(-?\d+)..(-?\d+)").unwrap();
}

type Vec2 = (isize, isize);

fn step(pos: Vec2, velocity: Vec2) -> (Vec2, Vec2) {
    let new_pos = (pos.0 + velocity.0, pos.1 + velocity.1);

    let new_dx = if velocity.0 == 0 { 0 } else { velocity.0 - 1 };
    let new_velocity = (new_dx, velocity.1 - 1);

    (new_pos, new_velocity)
}

fn test_velocity(
    mut velocity: Vec2,
    x_range: &RangeInclusive<isize>,
    y_range: &RangeInclusive<isize>,
    steps: isize,
) -> Option<isize> {
    let mut pos = (0, 0);
    let mut max_height = isize::MIN;
    let min_y = *y_range.start().min(y_range.end());
    let max_x = *x_range.start().max(x_range.end());
    for _ in 0..steps {
        if pos.0 > max_x || pos.1 < min_y {
            break;
        }
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
    let y_range = y1.min(y2)..=y2.max(y1);
    let max_steps = x1.max(x2).max(y1.abs()).max(y2.abs());

    let y = y_range.end().abs();

    let mut max_height = isize::MIN;
    for dx in 0..x2 {
        for dy in -y..y {
            if let Some(height) = test_velocity((dx, dy), &x_range, &y_range, max_steps) {
                max_height = std::cmp::max(max_height, height);
            }
        }
    }
    println!("{}", max_height);
}
