use itertools::Itertools;

macro_rules! puzzles {
    ($($mod_num:ident),* $(,)?) => {
        $(
            mod $mod_num;
        )*

        const PUZZLES: &[fn(Option<&str>)] = &[
            $(
                $mod_num::run,
            )*
        ];
    };
}

puzzles!(q1, q2, q3, q4, q5);

fn main() -> Result<(), String> {
    let puzzle_number = std::env::args()
        .nth(1)
        .ok_or("Missing puzzle number, usage: aoc-2025 <puzzle-number> [--stdin]")?
        .parse::<usize>()
        .map_err(|e| format!("Invalid puzzle number: {e}"))?;

    println!("Enter the input (enter newline twice to finish):");
    let inp = std::env::args().any(|arg| arg == "--stdin").then(|| {
        std::io::stdin()
            .lines()
            .map_while(Result::ok)
            .tuple_windows()
            .take_while(|(l1, l2)| !l1.is_empty() || !l2.is_empty())
            .map(|(l, _)| l)
            .collect::<Vec<_>>()
            .join("\n")
    });

    if puzzle_number == 0 || puzzle_number > PUZZLES.len() {
        return Err(format!(
            "Puzzle number out of range, must be between 1 and {}",
            PUZZLES.len()
        ));
    }

    PUZZLES[puzzle_number - 1](inp.as_ref().map(String::as_ref));

    Ok(())
}
