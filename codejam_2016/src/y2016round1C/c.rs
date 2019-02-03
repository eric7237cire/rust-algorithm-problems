use codejam::util::codejam::run_cases;

use itertools::Itertools;
use std::io::Write;
use bit_vec::BitVec;

/*

*/
pub fn solve_all_cases()
{
    run_cases(
        &[
            "C-small-practice",
            //"B-large-practice"
        ],
        "y2016round1C",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let nums = reader.read_num_line();

                if case_no != 1 {
                    //continue;
                }

                println!("Solving case {}", case_no);

                writeln!(buffer, "Case #{}: {}", case_no, solve(nums[0], nums[1], nums[2], nums[3])).unwrap();
            }
        },
    );
}


fn solve(J: usize, P: usize, S: usize, K: usize) -> String
{

    return format!("IMPOSSIBLE")




}
