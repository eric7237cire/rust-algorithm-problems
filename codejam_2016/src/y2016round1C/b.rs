use codejam::util::codejam::run_cases;

use itertools::Itertools;
use std::io::Write;

/*
Greedy algorithm
Maintain constraint
*/
pub fn solve_all_cases()
{
    run_cases(
        &["B-small-practice",
           // "A-large-practice"
             ],
        "y2016round1C",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let (B,M) = reader.read_tuple_2();

                let P = reader.read_num_line();
                assert_eq!(N, P.len());

                if case_no != 3 {
                    // continue;
                }

                //println!("Solving case {}", case_no);

                writeln!(buffer, "Case #{}: {}", case_no, solve(&P)).unwrap();
            }
        },
    );
}
