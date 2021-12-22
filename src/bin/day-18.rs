use aoc2021::commons::io::load_argv_lines;
use itertools::Itertools;

#[derive(Clone, Debug, Eq, PartialEq)]
struct NumElem {
    n: u32,
    depth: u16,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Num {
    d: Vec<NumElem>,
}

impl Num {
    pub fn add(&self, other: &Num) -> Num {
        let mut d = Vec::with_capacity(self.d.len() + other.d.len());
        d.extend(self.d.clone());
        d.extend(other.d.clone());
        for e in d.iter_mut() {
            e.depth += 1;
        }
        Num { d }
    }

    fn reduce_inner(&self) -> (Num, bool) {
        let mut new: Vec<NumElem> = self.d.clone();

        let mut has_acted = false;
        for i in 0..new.len() {
            let sub = &new[i];
            if sub.depth == 5 {
                let (left, right) = (new[i].n, new[i + 1].n);
                new[i] = NumElem { depth: 4, n: 0 };
                new.remove(i + 1);
                if i != 0 {
                    let before = new.get_mut(i - 1).unwrap();
                    before.n += left;
                }
                if let Some(after) = new.get_mut(i + 1) {
                    after.n += right;
                }
                has_acted = true;
                break;
            }
        }

        if !has_acted {
            for i in 0..new.len() {
                let sub = &new[i];
                if sub.n >= 10 {
                    let n = sub.n;
                    let depth = sub.depth + 1;
                    new[i] = NumElem { depth, n: (n / 2) };
                    new.insert(
                        i + 1,
                        NumElem {
                            depth,
                            n: (n + 1) / 2,
                        },
                    );
                    has_acted = true;
                    break;
                }
            }
        }

        (Num { d: new }, has_acted)
    }

    pub fn reduce(&self) -> Num {
        let mut result = self.clone();
        let mut modified = true;
        while modified {
            let t = result.reduce_inner();
            result = t.0;
            modified = t.1;
        }
        result
    }

    fn mag_inner(&self, i: &mut usize, depth: u16) -> u32 {
        let next = *i;
        let left = if self.d[next].depth == depth {
            *i += 1;
            self.d[next].n
        } else {
            self.mag_inner(i, depth + 1)
        };

        let next = *i;
        let right = if self.d[next].depth == depth {
            *i += 1;
            self.d[next].n
        } else {
            self.mag_inner(i, depth + 1)
        };

        (3 * left) + (2 * right)
    }

    pub fn magnitude(&self) -> u32 {
        let mut i = 0;
        self.mag_inner(&mut i, 1)
    }
}

fn main() {
    let mut numbers: Vec<Num> = Vec::new();
    for line in load_argv_lines::<String>() {
        let mut depth = 0;
        let mut number = Vec::new();
        for c in line.unwrap().chars() {
            match c {
                '[' => depth += 1,
                ',' => {}
                ']' => depth -= 1,
                _ => number.push(NumElem {
                    n: c.to_digit(10).unwrap(),
                    depth,
                }),
            }
        }
        numbers.push(Num { d: number });
    }

    let mut sum = numbers[0].clone();
    for num in numbers.iter().skip(1) {
        sum = sum.add(num).reduce();
    }
    println!("{:?}", sum.magnitude());

    let part2 = (0..numbers.len())
        .permutations(2)
        .map(|v| numbers[v[0]].add(&numbers[v[1]]).reduce().magnitude())
        .max()
        .unwrap();
    println!("{}", part2);
}
