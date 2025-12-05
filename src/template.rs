fn parse(inp: &str) -> () {
    unimplemented!()
}

fn solution_p1(inp: ()) -> u32 {
    unimplemented!()
}

fn solution_p2(inp: ()) -> u32 {
    unimplemented!()
}

pub fn run(inp: Option<&str>) {
    let inp = inp.unwrap_or(include_str!("../data/<number>.txt"));

    let parsed = parse(inp);
    println!("p1: {}", solution_p1(parsed));

    let parsed = parse(inp);
    println!("p2: {}", solution_p2(parsed));
}
