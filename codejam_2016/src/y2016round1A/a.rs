
use codejam::util::codejam::run_cases;

use std::io::Write;



/*

*/
pub fn solve_all_cases()
{
    run_cases(
        &["A-small-practice", "A-large-practice"],
        "y2016round1A",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let S = reader.read_string();

                if case_no != 3 {
                    // continue;
                }

                println!("Solving case {}", case_no);

                writeln!(
                    buffer,
                    "Case #{}: {}",
                    case_no,
                    solve(&S)
                )
                .unwrap();
            }
        },
    );
}

fn solve(S: &str) -> String
{
    let mut ans = String::new();

    ans.push(S.chars().next().unwrap());

    for ch in S.chars().skip(1) {
        if ch >= ans.chars().next().unwrap() {
            ans.insert(0, ch);
        } else {
            ans.push(ch);
        }

    }

    ans
}
