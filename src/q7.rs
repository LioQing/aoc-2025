use std::collections::{HashMap, HashSet};

fn parse(inp: &str) -> (usize, impl Iterator<Item = impl Iterator<Item = bool>>) {
    let mut lines = inp.lines();
    let start = lines.next().unwrap().find('S').unwrap();
    let rows = lines.map(|l| l.chars().map(|c| c == '^'));

    (start, rows)
}

fn solution_p1((start, rows): (usize, impl Iterator<Item = impl Iterator<Item = bool>>)) -> u32 {
    let beams = [start].into_iter().collect::<HashSet<_>>();

    rows.fold((beams, 0), |(beams, mut count), row| {
        let row = row.collect::<Vec<_>>();
        let new_beams = beams
            .into_iter()
            .flat_map(|beam| {
                if row[beam] {
                    // input does not underflow or overflow
                    count += 1;
                    vec![beam - 1, beam + 1]
                } else {
                    vec![beam]
                }
            })
            .collect::<HashSet<_>>();

        (new_beams, count)
    })
    .1
}

fn solution_p2((start, rows): (usize, impl Iterator<Item = impl Iterator<Item = bool>>)) -> u64 {
    let beams = [(start, 1)].into_iter().collect::<HashMap<_, _>>();

    rows.fold(beams, |beams, row| {
        let row = row.collect::<Vec<_>>();
        let mut new_beams = HashMap::new();

        beams.into_iter().for_each(|(beam, count)| {
            if row[beam] {
                new_beams
                    .entry(beam - 1)
                    .and_modify(|c| *c += count)
                    .or_insert(count);

                new_beams
                    .entry(beam + 1)
                    .and_modify(|c| *c += count)
                    .or_insert(count);
            } else {
                new_beams
                    .entry(beam)
                    .and_modify(|c| *c += count)
                    .or_insert(count);
            }
        });

        new_beams
    })
    .values()
    .sum()
}

pub fn run(inp: Option<&str>) {
    let inp = inp.unwrap_or(include_str!("../data/q7.txt"));

    let parsed = parse(inp);
    println!("p1: {}", solution_p1(parsed));

    let parsed = parse(inp);
    println!("p2: {}", solution_p2(parsed));
}
