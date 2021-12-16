use aoc2021::commons::io::load_argv_lines;

fn main() {
    let ints: Vec<u32> = load_argv_lines().map(|res| res.unwrap()).collect();
    // Part 1
    let mut last = 0;
    let mut increases = 0;
    for i in ints.clone() {
        if last != 0 && i > last {
            increases += 1;
        }
        last = i;
    }
    println!("{}", increases);

    // Part 2
    last = 0;
    increases = 0;
    for window in ints.windows(3) {
        let sum = window.iter().sum();
        if last != 0 && last < sum {
            increases += 1;
        }
        last = sum;
    }
    println!("{}", increases);
}
