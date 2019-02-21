use codejam::util::codejam::run_cases;
use std::io::Write;
use crate::y2008round2::a::Gate::*;

enum Gate {
    AND,
    OR
}

/*
Dynamic programming
Boolean tree
*/
pub fn solve_all_cases()
{
    run_cases(
        &["A-small-practice", "A-large-practice"],
        "y2008round2",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {

                let (m,v) : (usize, usize) = reader.read_tuple_2();

                let interior_nodes: Vec<(Gate, bool)> = (0..(m-1) / 2).map(|_| {
                    let (g, c) : (usize,usize) = reader.read_tuple_2();
                    (if g == 1 { AND } else { OR }, c == 1)
                }
                ).collect();

                let leaf_nodes: Vec<bool> = (0..(m+1) / 2).map( |_| {
                    let v : usize = reader.read_int();
                    v == 1
                }

                ).collect();

                if case_no != 3 {
                    // continue;
                }

                println!("Solving case {}", case_no);

                writeln!(
                    buffer,
                    "Case #{}: {}",
                    case_no,
                    if let Some(ans) = solve(v==1,interior_nodes.as_slice(), leaf_nodes.as_slice()) {
                        ans.to_string()
                    } else {
                        "IMPOSSIBLE".to_string()
                    }
                )
                .unwrap();
            }
        },
    );
}

fn solve(v: bool, interior_nodes: &[(Gate, bool)], leaf_nodes: &[bool]) -> Option<usize>
{
    None
}
