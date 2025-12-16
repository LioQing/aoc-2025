use std::collections::HashMap;

fn parse(inp: &str) -> HashMap<&str, Vec<&str>> {
    inp.lines()
        .map(|l| l.split_once(':').unwrap())
        .map(|(from, to)| {
            (
                from,
                to.split_whitespace().map(str::trim).collect::<Vec<_>>(),
            )
        })
        .collect()
}

fn solution_p1(inp: HashMap<&str, Vec<&str>>) -> u32 {
    let mut stack = vec!["you"];
    let mut count = 0;

    while let Some(curr) = stack.pop() {
        if curr == "out" {
            count += 1;
        } else {
            stack.extend(inp[curr].iter());
        }
    }

    count
}

fn solution_p2(inp: HashMap<&str, Vec<&str>>) -> u64 {
    // observing the input, `out` is the only sink node

    const NONE: usize = 0;
    const FFT: usize = 0b01;
    const DAC: usize = 0b10;
    const BOTH: usize = 0b11;
    const LEN: usize = 4;

    let mut stack = vec![("svr", false)];
    let mut visits = HashMap::<_, [_; LEN]>::new();

    while let Some((curr, waiting)) = stack.pop() {
        // println!("{}", stack.len());
        if waiting {
            // dfs guarantees children explore their whole tree
            visits.insert(
                curr,
                inp.get(curr)
                    .map(|iter| {
                        iter.iter().map(|&to| visits[&to]).fold(
                            std::array::from_fn(|_| 0),
                            |mut visit, visit_to| {
                                visit
                                    .iter_mut()
                                    .zip(visit_to.iter())
                                    .for_each(|(v, t)| *v += t);
                                visit
                            },
                        )
                    })
                    .unwrap_or({
                        let mut visit = [0; LEN];
                        visit[BOTH] = 1;
                        visit
                    }),
            );

            match curr {
                "fft" => {
                    visits.get_mut(curr).unwrap()[DAC] += visits[curr][BOTH];
                    visits.get_mut(curr).unwrap()[NONE] += visits[curr][FFT];
                }
                "dac" => {
                    visits.get_mut(curr).unwrap()[FFT] += visits[curr][BOTH];
                    visits.get_mut(curr).unwrap()[NONE] += visits[curr][DAC];
                }
                _ => (),
            }
        } else if !visits.contains_key(&curr) {
            // if we have not explored out children, do it for our parents
            stack.push((curr, true));
            stack.extend(
                inp.get(curr)
                    .unwrap_or(&vec![])
                    .iter()
                    .map(|&to| (to, false)),
            );
        }
    }

    visits["svr"][NONE]
}

pub fn run(inp: Option<&str>) {
    let inp = inp.unwrap_or(include_str!("../data/q11.txt"));

    let parsed = parse(inp);
    if parsed.contains_key("you") {
        println!("p1: {}", solution_p1(parsed));
    }

    let parsed = parse(inp);
    if parsed.contains_key("svr") {
        println!("p2: {}", solution_p2(parsed));
    }
}
