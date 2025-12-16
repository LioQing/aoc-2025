use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use ordered_float::OrderedFloat;
use rayon::iter::{ParallelBridge, ParallelIterator};

#[derive(Debug)]
struct Machine {
    lights: u32,
    buttons: Vec<u32>,
    button_indices: Vec<HashSet<usize>>,
    joltages: Vec<u32>,
}

fn parse(inp: &str) -> impl Iterator<Item = Machine> {
    inp.lines().map(|l| {
        let mut parts = l.split_whitespace();

        let lights_str = parts.next().unwrap();
        let lights = lights_str[1..lights_str.len() - 1]
            .chars()
            .rev()
            .fold(0, |l, c| (l << 1) | if c == '#' { 1 } else { 0 });

        let mut parts = parts.rev();

        let joltages_str = parts.next().unwrap();
        let joltages = joltages_str[1..joltages_str.len() - 1]
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        let parts = parts.rev();

        let button_indices = parts
            .map(|s| {
                s[1..s.len() - 1]
                    .split(',')
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<HashSet<_>>()
            })
            .collect::<Vec<_>>();

        let buttons = button_indices
            .iter()
            .map(|b| b.iter().fold(0, |l, i| l | (1 << i)))
            .collect::<Vec<_>>();

        Machine {
            lights,
            buttons,
            button_indices,
            joltages,
        }
    })
}

fn solution_p1(inp: impl Iterator<Item = Machine>) -> u32 {
    inp.map(|machine| {
        let mut queue = [(0, 0)].into_iter().collect::<VecDeque<_>>();
        let mut visited = HashSet::new();

        while let Some((count, curr)) = queue.pop_front() {
            if curr == machine.lights {
                return count;
            }

            visited.insert(curr);

            queue.extend(
                machine
                    .buttons
                    .iter()
                    .map(|b| (count + 1, curr ^ b))
                    .filter(|(_, next)| !visited.contains(next)),
            );
        }

        unreachable!()
    })
    .sum()
}

fn solution_p2(inp: impl Iterator<Item = Machine>) -> u32 {
    /// `m` is the matrix of `row * (col + 1)` where the extra column is the joltages
    fn reduce_row_echelon_form(m: &mut [Vec<f64>]) -> bool {
        // println!("Matrix");
        // debug_mat(m);

        let row = m.len();
        let col = m[0].len() - 1;
        let mut i = 0;
        while i < row.min(col) {
            // find pivot
            let pivot_row = m
                .iter()
                .enumerate()
                .skip(i)
                .min_set_by_key(|&(_, xs)| {
                    // find candidates with the same left-most non-zero position
                    xs.iter().position(|&x| x != 0f64).unwrap_or(usize::MAX)
                })
                .into_iter()
                .min_by_key(|&(_, xs)| {
                    // except the left-most position
                    // other positions should have as many zeros as possible
                    xs.iter()
                        .map(|x| x.abs())
                        .map(OrderedFloat)
                        .collect::<Vec<_>>()
                })
                .unwrap()
                .0;

            // find index of pivot's left-most position with non-zero coeff
            let Some(pivot_col) = m[pivot_row].iter().position(|&x| x != 0f64) else {
                break;
            };

            if pivot_col >= col {
                // `start` is at the joltage column
                break;
            }

            m.swap(i, pivot_row);

            // multiply pivot to obtain 1
            let factor = m[i][pivot_col];
            m[i].iter_mut().for_each(|x| *x /= factor);

            // elimination below
            for j in i + 1..row {
                let factor = m[j][pivot_col];
                for k in pivot_col..col + 1 {
                    m[j][k] -= m[i][k] * factor;
                }
            }

            i += 1;
        }

        // elimination above
        for j in (0..i).rev() {
            let pivot_col = m[j].iter().position(|&x| x != 0f64).unwrap();

            for k in 0..j {
                let factor = m[k][pivot_col];
                for l in pivot_col..col + 1 {
                    m[k][l] -= m[j][l] * factor;
                }
            }
        }

        i == col
    }

    #[allow(dead_code)]
    fn debug_mat(m: &[Vec<f64>]) {
        for row in m.iter() {
            for &val in row.iter() {
                if val != 0f64 {
                    print!("{val:>8.2} ");
                } else {
                    print!("{:>8} ", ".");
                }
            }
            println!();
        }
    }

    // let m = vec![
    //     vec![1, 3, 3, 8, 5],
    //     vec![0, 1, 3, 10, 8],
    //     vec![0, 0, 0, -1, -4],
    //     vec![0, 0, 0, 2, 8],
    // ];
    // let m = vec![
    //     vec![0, 1, 2, 1, -3],
    //     vec![1, 2, -1, 2, 5],
    //     vec![0, -5, 3, -5, -11],
    // ];
    // let m = vec![vec![3, -5, 1, 14], vec![1, -2, 1, 7], vec![2, -2, -1, 3]];

    // let mut m = m
    //     .into_iter()
    //     .map(|row| row.into_iter().map(|x| x as f64).collect::<Vec<_>>())
    //     .collect::<Vec<_>>();
    // reduce_row_echelon_form(m.as_mut_slice());
    // debug_mat(&m);

    inp.map(|machine| {
        // println!("{machine:?}");

        let mut m = machine
            .joltages
            .iter()
            .enumerate()
            .map(|(i, &j)| {
                machine
                    .button_indices
                    .iter()
                    .map(|b| if b.contains(&i) { 1f64 } else { 0f64 })
                    .chain(std::iter::once(j as f64))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let rref_complete = reduce_row_echelon_form(&mut m);
        // println!("RREF complete={rref_complete}");
        // debug_mat(&m);

        if rref_complete {
            m.into_iter()
                .map(|row| row.last().unwrap().round() as u32)
                .sum::<u32>()
        } else {
            let pivot_cols = m
                .iter()
                .flat_map(|row| row.iter().position(|&x| x == 1f64))
                .collect::<HashSet<_>>();

            let free_cols = (0..machine.buttons.len())
                .filter(|x| !pivot_cols.contains(x))
                .collect::<Vec<_>>();

            // build all the button presses from the free vars
            let presses_builder = |free_values: &[i32]| {
                let press_by_index = m
                    .iter()
                    .filter(|row| row.iter().any(|&x| x != 0f64))
                    .map(|row| {
                        let pivot_col = row.iter().position(|&x| x == 1f64).unwrap();
                        let value = *row.last().unwrap()
                            - free_cols
                                .iter()
                                .zip(free_values.iter())
                                .map(|(&col, &val)| row[col] * val as f64)
                                .sum::<f64>();

                        (pivot_col, value)
                    })
                    .chain(
                        free_cols
                            .iter()
                            .copied()
                            .zip(free_values.iter().map(|&x| x as f64)),
                    )
                    .map(|(i, x)| ((x - x.round()).abs() < 1e-3).then_some((i, x.round() as i32)))
                    .collect::<Option<HashMap<_, _>>>()?;

                Some(
                    (0..machine.buttons.len())
                        .map(|i| press_by_index[&i])
                        .collect::<Vec<_>>(),
                )
            };

            let press_counts = |presses: &[i32]| -> Option<u32> {
                presses
                    .iter()
                    .all(|&x| x >= 0)
                    .then(|| presses.iter().map(|&x| x as u32).sum())
            };

            std::iter::repeat_n(
                0..=machine.joltages.iter().max().map(|&x| x as i32).unwrap(),
                free_cols.len(),
            )
            .multi_cartesian_product()
            .par_bridge()
            .flat_map(|product| presses_builder(&product))
            .flat_map(|presses| press_counts(&presses))
            .min()
            // .inspect(|x| println!("{x}"))
            .unwrap()
        }
    })
    .sum()
}

pub fn run(inp: Option<&str>) {
    let inp = inp.unwrap_or(include_str!("../data/q10.txt"));

    let parsed = parse(inp);
    println!("p1: {}", solution_p1(parsed));

    let parsed = parse(inp);
    println!("p2: {}", solution_p2(parsed));
}
