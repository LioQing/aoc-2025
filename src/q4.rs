use std::collections::VecDeque;

use glam::*;
use itertools::Itertools;

struct Map2 {
    data: Vec<bool>,
    size: UVec2,
}

impl Map2 {
    fn coords_to_index(&self, coords: IVec2) -> usize {
        (coords.y * self.size.x as i32 + coords.x) as usize
    }

    fn get(&self, coords: IVec2) -> Option<bool> {
        if coords.cmplt(IVec2::ZERO).any() || coords.cmpge(self.size.as_ivec2()).any() {
            return None;
        }

        self.data.get(self.coords_to_index(coords)).copied()
    }

    fn remove(&mut self, coords: IVec2) {
        let index = self.coords_to_index(coords);
        self.data[index] = false;
    }

    fn can_be_removed(&self, coords: IVec2) -> bool {
        self.get(coords).unwrap_or(false)
            && neighbors(coords)
                .filter(|neighbor| self.get(*neighbor).unwrap_or(false))
                .count()
                < 4
    }
}

impl std::fmt::Display for Map2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for coords in (0..self.size.y)
            .cartesian_product(0..self.size.x)
            .map(|(y, x)| IVec2::new(x as i32, y as i32))
        {
            if self.can_be_removed(coords) {
                write!(f, "X")?;
            } else {
                write!(f, "{}", if self.get(coords).unwrap() { '@' } else { '.' })?;
            }

            if coords.x == self.size.x as i32 - 1 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

fn neighbors(coords: IVec2) -> impl Iterator<Item = IVec2> {
    const DIRS: &[IVec2] = &[
        IVec2::new(1, 0),
        IVec2::new(1, 1),
        IVec2::new(0, 1),
        IVec2::new(-1, 1),
        IVec2::new(-1, 0),
        IVec2::new(-1, -1),
        IVec2::new(0, -1),
        IVec2::new(1, -1),
    ];
    DIRS.iter().map(move |dir| coords + dir)
}

fn parse(inp: &str) -> Map2 {
    let width = inp.lines().next().unwrap().len() as u32;
    let mut height = 0;
    let data = inp
        .lines()
        .inspect(|_| height += 1)
        .flat_map(|l| l.chars().map(|c| c == '@'))
        .collect::<Vec<_>>();

    Map2 {
        data,
        size: UVec2::new(width, height),
    }
}

fn solution_p1(inp: Map2) -> u32 {
    // println!("{inp}");

    (0..inp.size.y)
        .cartesian_product(0..inp.size.x)
        .map(|(y, x)| IVec2::new(x as i32, y as i32))
        .filter(|coords| inp.can_be_removed(*coords))
        .count() as u32
}

fn solution_p2(mut inp: Map2) -> u32 {
    let mut stack = (0..inp.size.y)
        .cartesian_product(0..inp.size.x)
        .map(|(y, x)| IVec2::new(x as i32, y as i32))
        .filter(|coords| inp.can_be_removed(*coords))
        .collect::<VecDeque<_>>();
    let mut count = 0;

    while let Some(curr) = stack.pop_front() {
        // println!("{curr}");
        // println!("{inp}");
        if inp.get(curr) == Some(false) {
            continue;
        }

        count += 1;
        inp.remove(curr);
        stack.extend(neighbors(curr).filter(|neighbor| inp.can_be_removed(*neighbor)));
    }

    count
}

pub fn run(inp: Option<&str>) {
    let inp = inp.unwrap_or(include_str!("../data/q4.txt"));

    let parsed = parse(inp);
    println!("p1: {}", solution_p1(parsed));

    let parsed = parse(inp);
    println!("p2: {}", solution_p2(parsed));
}
