use commons::io::load_stdin_lines;
use itertools::Itertools;

fn main() {
    let input: Vec<String> = load_stdin_lines().map(|x| x.unwrap()).collect();

    let mut part1 = 0;
    let mut part2_scores = Vec::new();

    for line in input {
        let mut bracket_stack = Vec::new();
        let mut corrupted = false;
        for c in line.chars() {
            let wanted = bracket_stack.last();
            let ok = match (c, wanted) {
                ('(', _) => {
                    bracket_stack.push(')');
                    true
                }
                ('[', _) => {
                    bracket_stack.push(']');
                    true
                }
                ('{', _) => {
                    bracket_stack.push('}');
                    true
                }
                ('<', _) => {
                    bracket_stack.push('>');
                    true
                }
                (')' | ']' | '}' | '>', Some(&c1)) => {
                    if c1 != c {
                        false
                    } else {
                        bracket_stack.pop();
                        true
                    }
                }
                (')' | ']' | '}' | '>', None) => false,
                _ => {
                    panic!("BAD STATE!");
                }
            };

            if !ok {
                part1 += match c {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => panic!("Illegal character not expected"),
                };
                corrupted = true;
                break;
            }
        }

        if !corrupted {
            let mut score: u64 = 0;
            for c in bracket_stack.iter().rev() {
                score = score * 5
                    + match c {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => panic!("Illegal character not expected"),
                    };
            }
            part2_scores.push(score);
        }
    }
    println!("{:?}", part1);
    println!(
        "{:?}",
        part2_scores
            .iter()
            .sorted()
            .nth(part2_scores.len() / 2)
            .unwrap()
    );
}
