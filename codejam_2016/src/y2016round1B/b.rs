use codejam::util::codejam::run_cases;

use itertools::Itertools;
use std::collections::HashMap;
use std::io::Write;

/*

*/
pub fn solve_all_cases()
{
    run_cases(
        &["B-small-practice",
            //"A-large-practice"
            ],
        "y2016round1B",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let scores = reader.read_string_line();

                assert_eq!(scores.len(), 2, "{}",scores[0]);

                if case_no != 3 {
                    // continue;
                }

                //println!("Solving case {}", case_no);

                writeln!(buffer, "Case #{}: {}", case_no, solve(&scores[0], &scores[1])).unwrap();
            }
        },
    );
}

fn solve(C: &str, J: &str) -> String
{
    format!("hey")
}
