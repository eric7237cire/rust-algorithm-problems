
use codejam::util::codejam::run_cases;

use std::io::Write;
use itertools::Itertools;


/*

*/
pub fn solve_all_cases()
{
    run_cases(
        &["B-small-practice", 
        //"A-large-practice"
        ],
        "y2016round1A",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let N : u16 = reader.read_int();

                let papers = (0..2*N - 1).map( |_| reader.read_num_line()).collect::<Vec<_>>();

                if case_no != 3 {
                    // continue;
                }

                println!("Solving case {}", case_no);

                writeln!(
                    buffer,
                    "Case #{}: {}",
                    case_no,
                    solve(&papers).iter().join(" ")
                )
                .unwrap();
            }
        },
    );
}

fn solve(papers: &[Vec<u16>]) -> Vec<u16>
{
   
   let mut ans = Vec::new();
   ans.push(3);
   ans.push(5);

   ans
}
