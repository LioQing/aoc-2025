use itertools::Itertools;

macro_rules! InpIter {
    () => {
        impl Iterator<Item = RowIter<
            impl Iterator<Item = u64>,
            impl Iterator<Item = Op>
        >>
    }
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Mul,
}

impl Op {
    fn new(s: char) -> Option<Self> {
        match s {
            '+' => Some(Self::Add),
            '*' => Some(Self::Mul),
            _ => None,
        }
    }

    fn get_fn(self) -> fn(u64, u64) -> u64 {
        match self {
            Op::Add => |x, y| x + y,
            Op::Mul => |x, y| x * y,
        }
    }

    fn id_elem(self) -> u64 {
        match self {
            Op::Add => 0,
            Op::Mul => 1,
        }
    }
}

#[derive(Debug)]
enum RowIter<NumIter: Iterator<Item = u64>, OpIter: Iterator<Item = Op>> {
    Nums(NumIter),
    Ops(OpIter),
}

fn parse(inp: &str) -> InpIter!() {
    inp.lines().map(|l| {
        let mut items = l.split_whitespace();
        match items.next().unwrap() {
            item if item.parse::<u64>().is_ok() => RowIter::Nums(
                std::iter::once(item)
                    .chain(items)
                    .map(|item| item.parse().unwrap()),
            ),
            item if item == "*" || item == "+" => RowIter::Ops(
                std::iter::once(item)
                    .chain(items)
                    .map(|item| Op::new(item.chars().next().unwrap()).unwrap()),
            ),
            _ => unreachable!(),
        }
    })
}

fn solution_p1(inp: InpIter!()) -> u64 {
    let mut inp_iter = inp.collect::<Vec<_>>().into_iter().rev();
    let acc = inp_iter
        .next()
        .map(|ops| {
            let RowIter::Ops(ops) = ops else {
                unreachable!()
            };

            ops.map(|op| (op.id_elem(), op.get_fn()))
                .collect::<Vec<_>>()
        })
        .unwrap();

    inp_iter
        .fold(acc, |mut acc, nums| {
            let RowIter::Nums(nums) = nums else {
                unreachable!()
            };

            for ((acc, f), n) in acc.iter_mut().zip_eq(nums) {
                *acc = f(*acc, n);
            }

            acc
        })
        .into_iter()
        .map(|(acc, _)| acc)
        .sum()
}

fn solution_p2(inp: &str) -> u64 {
    let mut lines = inp.lines().collect::<Vec<_>>();
    let col_num = lines[0].len();
    let ops = lines
        .pop()
        .unwrap()
        .split_whitespace()
        .map(|op| Op::new(op.chars().next().unwrap()).unwrap());
    let problems = lines
        .into_iter()
        .fold(vec![0u64; col_num], |mut nums, l| {
            nums.iter_mut().zip(l.chars()).for_each(|(n, l)| {
                if let Some(x) = l.to_digit(10) {
                    *n = *n * 10 + x as u64
                }
            });
            nums
        })
        .into_iter()
        .fold(vec![vec![]], |mut nums, n| {
            if n == 0 {
                // 0 imply it is the space line in between problems
                nums.push(Vec::new());
            } else {
                nums.last_mut().unwrap().push(n);
            }
            nums
        });

    problems
        .into_iter()
        .zip_eq(ops)
        .map(|(nums, op)| {
            nums.into_iter()
                .fold(op.id_elem(), |acc, n| op.get_fn()(acc, n))
        })
        // .inspect(|x| println!("{x}"))
        .sum()
}

pub fn run(inp: Option<&str>) {
    let inp = inp.unwrap_or(include_str!("../data/q6.txt"));

    let parsed = parse(inp);
    println!("p1: {}", solution_p1(parsed));

    println!("p2: {}", solution_p2(inp));
}
