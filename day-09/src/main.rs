use commons::grid::{Grid, SparseGrid};
use commons::io::load_file_lines;

fn main() {
    let mut grid = SparseGrid::new();
    let adjacent: Vec<(isize, isize)> = vec![
        (0, -1),
        (0, 1),
        (1, 0),
        (-1, 0),
    ];

    for (y, line) in load_file_lines::<String>("input.txt").enumerate() {
        for (x, c) in line.unwrap().chars().enumerate() {
            grid.set(
                (x as isize, y as isize),
                c.to_string().parse::<u8>().unwrap(),
            );
        }
    }

    let mut sum = 0;
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let value = grid.at(&(x as isize, y as isize)).unwrap();
            let surrounding_greater = adjacent
                .iter()
                .map(|(x1, y1)| {
                    grid.at(&(x as isize + x1, y as isize + y1))
                        .copied()
                        .unwrap_or(u8::MAX)
                })
                .filter(|v1| v1 > value)
                .count();
            if surrounding_greater == 4 {
                sum += *value as u32 + 1;
            }
        }
    }
    println!("{}", sum);
}
