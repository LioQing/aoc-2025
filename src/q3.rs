fn parse(inp: &str) -> impl Iterator<Item = impl Iterator<Item = i64>> {
    inp.lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i64))
}

fn solution_p1(inp: impl Iterator<Item = impl Iterator<Item = i64>>) -> i64 {
    inp.map(|l| l.collect::<Vec<_>>())
        .map(|v| {
            v.iter()
                .copied()
                .enumerate()
                .skip(1)
                .fold((-1, [v[0], -1]), |(mut max, mut digits), (i, x)| {
                    if i < v.len() - 1 && x > digits[0] {
                        digits = [x, -1];
                    } else if x > digits[1] {
                        digits[1] = x;
                    }

                    if digits[1] != -1 {
                        max = max.max(digits[0] * 10 + digits[1]);
                    }

                    // println!("{i} {x} {digits:?} {max}");

                    (max, digits)
                })
                .0
        })
        // .inspect(|x| println!("max: {x}"))
        .sum()
}

fn solution_p2(inp: impl Iterator<Item = impl Iterator<Item = i64>>) -> i64 {
    inp.map(|l| l.collect::<Vec<_>>())
        .map(|v| {
            let mut digits = Vec::new();
            digits.resize(12, -1);
            digits[0] = v[0];

            v.iter()
                .copied()
                .enumerate()
                .skip(1)
                .fold((-1, digits), |(mut max, mut digits), (i, x)| {
                    for j in 0..digits.len() {
                        // println!("{i} {} {}", v.len(), digits.len() - j - 1);
                        if i < v.len() - (digits.len() - j - 1) && x > digits[j] {
                            digits[j] = x;
                            for d in digits.iter_mut().skip(j + 1) {
                                *d = -1;
                            }
                            break;
                        }
                    }

                    if !digits.contains(&-1) {
                        max = max.max(
                            digits
                                .iter()
                                .rev()
                                .enumerate()
                                .map(|(j, &d)| d * 10i64.pow(j as u32))
                                .sum(),
                        )
                    }

                    // println!("{i} {x} {digits:?} {max}");

                    (max, digits)
                })
                .0
        })
        // .inspect(|x| println!("max: {x}"))
        .sum()
}

pub fn run(inp: Option<&str>) {
    let inp = inp.unwrap_or(include_str!("../data/q3.txt"));

    let parsed = parse(inp);
    println!("p1: {}", solution_p1(parsed));

    let parsed = parse(inp);
    println!("p2: {}", solution_p2(parsed));
}
