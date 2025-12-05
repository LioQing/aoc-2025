use std::ops::Range;

use itertools::Itertools;

struct Db {
    freshes: Vec<Range<u64>>,
    ingredients: Vec<u64>,
}

fn parse(inp: &str) -> Db {
    let mut lines = inp.lines();
    let freshes = lines
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            l.split_once('-')
                .map(|(start, end)| start.parse().unwrap()..end.parse::<u64>().unwrap() + 1)
                .unwrap()
        })
        .collect::<Vec<_>>();
    let ingredients = lines.map(|l| l.parse().unwrap()).collect::<Vec<_>>();

    Db {
        freshes,
        ingredients,
    }
}

fn solution_p1(inp: Db) -> u32 {
    inp.ingredients
        .iter()
        .filter(|ing| inp.freshes.iter().any(|fresh| fresh.contains(ing)))
        .count() as u32
}

fn solution_p2(inp: Db) -> u64 {
    inp.freshes
        .into_iter()
        .sorted_by_key(|fresh| fresh.start)
        .chain(std::iter::once(u64::MAX..u64::MAX))
        .fold((0, 0..0), |(mut count, mut curr), next| {
            if curr.end < next.start {
                // println!("{curr:?} {next:?}");
                count += curr.end - curr.start;
                curr = next;

                return (count, curr);
            }

            if next.end > curr.end {
                curr.end = next.end;
            }

            (count, curr)
        })
        .0
}

pub fn run(inp: Option<&str>) {
    let inp = inp.unwrap_or(include_str!("../data/q5.txt"));

    let parsed = parse(inp);
    println!("p1: {}", solution_p1(parsed));

    let parsed = parse(inp);
    println!("p2: {}", solution_p2(parsed));
}
