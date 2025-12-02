use std::ops::RangeInclusive;

struct Input {
    range: RangeInclusive<u64>,
    start_len: usize,
}

fn parse(inp: &str) -> impl Iterator<Item = Input> {
    inp.split(",")
        .map(str::trim)
        .filter(|range| !range.is_empty())
        .map(|range| -> [(u64, usize); 2] {
            range
                .split("-")
                .map(|n| (n.parse().unwrap(), n.len()))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .map(|[(start, start_len), (end, _)]| Input {
            range: start..=end,
            start_len,
        })
}

fn solution_p1(inp: impl Iterator<Item = Input>) -> u64 {
    inp.map(|inp| {
        let (mut curr, len) = if inp.start_len % 2 == 0 {
            (*inp.range.start(), inp.start_len as u32)
        } else {
            let len = inp.start_len as u32 + 1;
            (10u64.pow(len - 1), len)
        };

        let mut denom = 10u64.pow(len / 2);
        let mut len_bound = 10u64.pow(len);

        let mut sum = 0;
        // println!("{curr} {len} {denom} {len_bound}");
        while curr <= *inp.range.end() {
            if curr % denom == curr / denom {
                // println!("{curr}");
                sum += curr;
            }

            if curr == len_bound {
                curr = len_bound * 10;
                len_bound = curr * 10;
                denom *= 10;
            } else {
                curr += 1;
            }
        }

        sum
    })
    .sum()
}

fn solution_p2(inp: impl Iterator<Item = Input>) -> u64 {
    inp.map(|inp| {
        let mut len = inp.start_len as u32;
        let mut len_bound = 10u64.pow(len);

        let mut sum = 0;
        for curr in inp.range {
            if curr == len_bound {
                len_bound *= 10;
                len += 1;
            }

            for i in 2..=len {
                if !len.is_multiple_of(i) {
                    continue;
                }

                let first_denom = 10u64.pow(len / i);
                let first = curr % first_denom;

                if (1..i)
                    .map(|j| 10u64.pow(len / i * j))
                    .all(|denom| first == curr / denom % first_denom)
                {
                    // println!("{curr}");
                    sum += curr;
                    break;
                }
            }
        }

        sum
    })
    .sum()
}

pub fn run(inp: Option<&str>) {
    let inp = inp.unwrap_or(include_str!("../data/q2.txt"));

    let parsed = parse(inp);
    println!("p1: {}", solution_p1(parsed));

    let parsed = parse(inp);
    println!("p2: {}", solution_p2(parsed));
}
