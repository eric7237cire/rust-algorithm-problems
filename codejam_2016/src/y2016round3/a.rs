use codejam::util::codejam::run_cases;

//use itertools::Itertools;
use std::io::Write;

/*
Trees, binary tree representation
*/
pub fn solve_all_cases()
{
    run_cases(
        &["A-small-practice", "A-large-practice"],
        "y2016round3",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let s = reader.read_string();

                if case_no != 3 {
                    // continue;
                }

                println!("Solving case {}", case_no);

                writeln!(
                    buffer,
                    "Case #{}: {}",
                    case_no,
                    solve(&s)
                )
                .unwrap();
            }
        },
    );
}

fn solve(s: &str) -> usize
{
    let mut stack:Vec<char> = Vec::new();
    let mut score = 0;
    for ch in s.chars() {
        if let Some(top) = stack.last() {
            if *top == ch {
                score += 10;
                stack.pop();
                continue;
            }
        }
        stack.push(ch);

    }

    score + 5 * stack.len() / 2
}
