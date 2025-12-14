use std::{
    collections::HashMap,
    io::{BufRead, Write},
};

use glam::*;
use itertools::{FoldWhile, Itertools};

fn parse(inp: &str) -> impl Iterator<Item = UVec3> {
    inp.lines().map(|l| {
        l.split(',')
            .map(|c| c.parse().unwrap())
            .collect_array()
            .unwrap()
            .into()
    })
}

fn solution_p1(inp: impl Iterator<Item = UVec3>, max_count: usize) -> u32 {
    let components = inp
        .enumerate()
        .map(|(i, c)| (c, i))
        .collect::<HashMap<_, _>>();

    components
        .keys()
        .copied()
        .combinations(2)
        .map(|v| (v[0], v[1]))
        .sorted_by_key(|(a, b)| (a.as_i64vec3() - b.as_i64vec3()).length_squared())
        .fold_while((components, 0), |(mut components, count), (a, b)| {
            let Some(&comp_a) = components.get(&a) else {
                unreachable!()
            };
            let Some(&comp_b) = components.get(&b) else {
                unreachable!()
            };

            if comp_a != comp_b {
                components
                    .values_mut()
                    .filter(|c| **c == comp_b)
                    .for_each(|c| *c = comp_a);
            }

            if count + 1 == max_count {
                FoldWhile::Done((components, count + 1))
            } else {
                FoldWhile::Continue((components, count + 1))
            }
        })
        .into_inner()
        .0
        .into_values()
        .counts()
        .into_values()
        .sorted()
        .rev()
        .take(3)
        // .inspect(|v| println!("{v}"))
        .map(|v| v as u32)
        .product()
}

fn solution_p2(inp: impl Iterator<Item = UVec3>) -> u64 {
    let mut components = inp
        .enumerate()
        .map(|(i, c)| (c, i))
        .collect::<HashMap<_, _>>();

    components
        .keys()
        .copied()
        .combinations(2)
        .map(|v| (v[0], v[1]))
        .sorted_by_key(|(a, b)| (a.as_i64vec3() - b.as_i64vec3()).length_squared())
        .map_while(|(a, b)| {
            if components.values().all_equal() {
                return None;
            }

            let Some(&comp_a) = components.get(&a) else {
                unreachable!()
            };
            let Some(&comp_b) = components.get(&b) else {
                unreachable!()
            };

            if comp_a != comp_b {
                components
                    .values_mut()
                    .filter(|c| **c == comp_b)
                    .for_each(|c| *c = comp_a);
            }

            Some((a, b))
        })
        .last()
        .map(|(a, b)| a.x as u64 * b.x as u64)
        .unwrap()
}

pub fn run(inp: Option<&str>) {
    let max_count = if inp.is_some() {
        print!("Enter number of connection: ");
        std::io::stdout().flush().unwrap();
        let mut max_count_inp = String::new();
        std::io::stdin()
            .lock()
            .read_line(&mut max_count_inp)
            .unwrap();
        max_count_inp.trim().parse().unwrap()
    } else {
        1000
    };

    let inp = inp.unwrap_or(include_str!("../data/q8.txt"));

    let parsed = parse(inp);
    println!("p1: {}", solution_p1(parsed, max_count));

    let parsed = parse(inp);
    println!("p2: {}", solution_p2(parsed));
}
