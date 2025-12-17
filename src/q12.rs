use std::collections::HashMap;

use glam::*;
use itertools::Itertools;

struct Inp {
    shapes: Vec<[[bool; 3]; 3]>,
    regions: Vec<(UVec2, Vec<usize>)>,
}

fn parse(inp: &str) -> Inp {
    let mut lines = inp.lines().peekable();
    let shapes = lines
        .peeking_take_while(|l| !l.contains('x'))
        .chunks(5)
        .into_iter()
        .map(|chunk| {
            chunk
                .skip(1)
                .take(3)
                .map(|row| {
                    row.chars()
                        .take(3)
                        .map(|c| c == '#')
                        .collect_array()
                        .unwrap()
                })
                .collect_array()
                .unwrap()
        })
        .collect::<Vec<_>>();
    let regions = lines
        .map(|l| l.split_once(':').unwrap())
        .map(|(region, quantities)| {
            let (width, length) = region.split_once('x').unwrap();
            let (width, length) = (width.parse().unwrap(), length.parse().unwrap());
            let quantities = quantities
                .split_whitespace()
                .map(str::trim)
                .map(str::parse)
                .map(Result::unwrap)
                .collect::<Vec<_>>();

            (UVec2::new(width, length), quantities)
        })
        .collect::<Vec<_>>();

    Inp { shapes, regions }
}

fn solution_p1(inp: Inp) -> usize {
    println!("{:?}", inp.regions);
    inp.regions
        .into_iter()
        .filter(|(region, quantities)| {
            let present_size = quantities
                .iter()
                .zip(inp.shapes.iter())
                .map(|(&q, &s)| (q * s.iter().flatten().copied().filter(|&b| b).count()) as u32)
                .sum::<u32>();

            let region_size = region.x * region.y;

            println!("{region} {region_size} >= {present_size}");

            region_size >= present_size
        })
        .count()
}

fn solution_p2(inp: Inp) -> u32 {
    unimplemented!()
}

pub fn run(inp: Option<&str>) {
    let inp = inp.unwrap_or(include_str!("../data/q12.txt"));

    let parsed = parse(inp);
    println!("p1: {}", solution_p1(parsed));

    let parsed = parse(inp);
    println!("p2: {}", solution_p2(parsed));
}
