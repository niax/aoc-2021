use commons::io::load_file_lines;

fn main() {
    let ints: Vec<u32> = load_file_lines("input.txt")
        .map(|res| res.unwrap())
        .collect();
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
        let sum = window[0] + window[1] + window[2];
        if last != 0 && last < sum {
            increases += 1;
        }
        last = sum;
    }
    println!("{}", increases);
}
