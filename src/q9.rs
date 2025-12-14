use glam::*;
use itertools::Itertools;

fn parse(inp: &str) -> impl Iterator<Item = UVec2> {
    inp.lines().map(|l| {
        l.split(',')
            .map(|c| c.parse().unwrap())
            .collect_array()
            .unwrap()
            .into()
    })
}

fn solution_p1(inp: impl Iterator<Item = UVec2>) -> u64 {
    inp.combinations(2)
        .map(|v| (v[0], v[1]))
        .map(|(a, b)| (a.x.abs_diff(b.x) as u64 + 1) * (a.y.abs_diff(b.y) as u64 + 1))
        .max()
        .unwrap()
}

fn solution_p2(inp: impl Iterator<Item = UVec2>) -> u64 {
    let points = inp.collect::<Vec<_>>();

    points
        .iter()
        .tuple_combinations()
        .filter(|&(a, b)| {
            let top_left = a.min(*b);
            let bottom_right = a.max(*b);
            points.iter().tuple_windows().all(|(c, d)| {
                if c.x == d.x {
                    c.x >= top_left.x
                        || c.x <= bottom_right.x
                        || (c.y <= top_left.y && d.y <= top_left.y)
                        || (c.y >= bottom_right.y && d.y >= bottom_right.y)
                } else {
                    c.y >= bottom_right.y
                        || c.y <= top_left.y
                        || (c.x <= top_left.x && d.x <= top_left.x)
                        || (c.x >= bottom_right.x && d.x >= bottom_right.x)
                }
            })
        })
        .map(|(a, b)| (a.x.abs_diff(b.x) as u64 + 1) * (a.y.abs_diff(b.y) as u64 + 1))
        .max()
        .unwrap()
}

pub fn run(inp: Option<&str>) {
    let inp = inp.unwrap_or(include_str!("../data/q9.txt"));

    let parsed = parse(inp);
    println!("p1: {}", solution_p1(parsed));

    let parsed = parse(inp);
    println!("p2: {}", solution_p2(parsed));
}
