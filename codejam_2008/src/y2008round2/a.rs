use codejam::util::codejam::run_cases;
use std::io::Write;
use crate::y2008round2::a::Gate::*;
use std::cmp::min;

enum Gate {
    AND = 1,
    OR = 0
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
                    ( g as Gate, c == 1)
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
                    if let Some(ans) = solve(v==1,m,interior_nodes.as_slice(), leaf_nodes.as_slice()) {
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

fn solve(v: bool, m: usize, interior_nodes: &[(Gate, bool)], leaf_nodes: &[bool]) -> Option<usize>
{
    //dp[x] = min number of changes for x to be true
    let mut dp = vec![None; m];

    for (i, ln) in leaf_nodes.iter().enumerate()
        {
            //if v is false we switch the input values
            dp[i + interior_nodes.len()] = if v==ln { Some(1) } else { None} ;
        }
    //2n + 1; 2n + 2

    for (i, int_node) in interior_nodes.iter().enumerate()
        {
            let gate: Gate = if v { int_node.0 } else { (1-int_node.0) as Gate };
            let changable = int_node.1;
            let mut min_cost = usize::MAX;

            if gate == AND {
                if let (Some(lhs), Some(rhs)) = (dp[2 * i + 1], dp[2 * i + 2]) {
                    min_cost = lhs + rhs;
                }
            }

            if gate == OR || changable {
                let cost = if gate == OR {0} else {1};
                if let Some(lhs) = dp[2*i+1] {
                    min_cost = min(min_cost, lhs+cost);
                }

                if let Some(rhs) = dp[2*i+2] {
                    min_cost = min(min_cost, rhs+cost);
                }
            }

        }
    None
}
