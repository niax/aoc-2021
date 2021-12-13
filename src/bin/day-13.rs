use aoc2021::commons::{
    grid::{BitGrid, Grid},
    io::load_stdin_lines,
};

enum Fold {
    X(usize),
    Y(usize),
}

impl Fold {
    pub fn map_coord(&self, coord: (usize, usize)) -> (usize, usize) {
        let (x, y) = coord;
        match *self {
            Fold::X(pos) => {
                if x > pos {
                    let diff = x - pos;
                    (pos - diff, y)
                } else {
                    (x, y)
                }
            }
            Fold::Y(pos) => {
                if y > pos {
                    let diff = y - pos;
                    (x, pos - diff)
                } else {
                    (x, y)
                }
            }
        }
    }
}

fn render_points(points: &[(usize, usize)]) -> BitGrid {
    let max_x = points.iter().map(|p| p.0).max().unwrap() + 1;
    let max_y = points.iter().map(|p| p.1).max().unwrap() + 1;

    let mut grid = BitGrid::new(max_x, max_y);
    for p in points {
        grid.set(*p, true);
    }
    grid
}

fn main() {
    let mut points = Vec::new();
    let mut folds = Vec::new();
    let mut points_done = false;
    for line in load_stdin_lines::<String>() {
        let line = line.unwrap();
        if line.is_empty() {
            points_done = true;
        } else if !points_done {
            let (left, right) = line.split_once(',').unwrap();
            let point = (
                left.parse::<usize>().unwrap(),
                right.parse::<usize>().unwrap(),
            );
            points.push(point);
        } else {
            let part = line.split_whitespace().nth(2).unwrap();
            let (orientation, pos) = part.split_once('=').unwrap();
            let pos = pos.parse::<usize>().unwrap();
            let fold = match orientation {
                "x" => Fold::X(pos),
                "y" => Fold::Y(pos),
                _ => panic!("Bad fold type: {}", orientation),
            };
            folds.push(fold);
        }
    }
    for (i, fold) in folds.iter().enumerate() {
        let next_points = points.iter().map(|p| fold.map_coord(*p)).collect();
        points = next_points;
        if i == 0 {
            let grid = render_points(&points);
            println!("{}", grid.set_cell_count());
        }
    }
    let grid = render_points(&points);
    println!("{}", grid.decode_string());
    grid.print('#', ' ');
}
