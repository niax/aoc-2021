use commons::io::load_file_lines;

fn main() {
    let input: Vec<String> = load_file_lines("input.txt").map(|x| x.unwrap()).collect();

    let mut part1 = 0;

    for line in input {
        let mut bracket_stack = Vec::new();
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
                break;
            }
        }
    }
    println!("{:?}", part1);
}
