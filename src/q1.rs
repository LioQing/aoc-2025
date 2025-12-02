fn parse(inp: &str) -> impl Iterator<Item = (i32, i32)> {
    inp.lines().map(move |l| {
        let mut iter = l.chars();

        let sign = if iter.next() == Some('L') { -1 } else { 1 };

        let clicks = iter.collect::<String>().trim().parse().unwrap();

        (sign, clicks)
    })
}

fn solution_p1(inp: impl Iterator<Item = (i32, i32)>) -> i32 {
    inp.map(|(sign, clicks)| sign * clicks)
        .fold((50, 0), |(mut curr, mut count), offset| {
            curr += offset;
            curr = curr.rem_euclid(100);

            if curr == 0 {
                count += 1;
            }

            (curr, count)
        })
        .1
}

fn solution_p2(inp: impl Iterator<Item = (i32, i32)>) -> i32 {
    inp.map(|(sign, clicks)| sign * clicks)
        .fold((50, 0), |(mut curr, mut count), offset| {
            curr += offset;

            if offset < 0 && curr == 0 {
                // When curr is on 0, going negative does not click on 0
                curr += 100;
            }

            curr += offset;

            let mut q = curr.div_euclid(100).abs();

            curr = curr.rem_euclid(100);

            if offset < 0 && curr == 0 {
                // After going negative, if curr is on 0 it count as a click on 0
                q += 1;
            }

            count += q;

            // println!("{count} {q:>2} {offset:>3} {curr:>2}");

            (curr, count)
        })
        .1
}

pub fn run(inp: Option<&str>) {
    let inp = inp.unwrap_or(include_str!("../data/q1.txt"));

    let parsed = parse(inp);
    println!("p1: {}", solution_p1(parsed));

    let parsed = parse(inp);
    println!("p2: {}", solution_p2(parsed));
}
