use codejam::util::codejam::run_cases;
use std::io::Write;
use bit_vec::BitVec;
use itertools::Itertools;

/*

*/
pub fn solve_all_cases()
{
    run_cases(
        &["B-small-practice", "B-large-practice"],
        "y2008round1a",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {


	            let num_flavors = reader.read_int();
                let num_customers = reader.read_int();

                let cust_data: Vec<Vec<u32>> = (0..num_customers).map(|_| reader.read_num_line()).collect();

                if case_no != 3 {
                    // continue;
                }

                println!("Solving case {}", case_no);

                writeln!(
                    buffer,
                    "Case #{}: {}",
                    case_no,
                    if let Some(ans) = solve(
                        num_flavors, cust_data.as_slice()
                    ) {
                        ans.iter().map(|b| if b { '1' } else {'0'}).join(" ")
                    } else {
                        "IMPOSSIBLE".to_string()
                    }
                )
                .unwrap();
            }
        },
    );
}

fn solve(num_flavors: usize, cust_data: &[Vec<u32>]) -> Option<BitVec>
{
    None
}
