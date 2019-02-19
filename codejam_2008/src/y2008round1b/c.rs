use codejam::util::codejam::run_cases;
use std::io::Write;
use itertools::Itertools;

/*
Binary Interval tree using an array
*/
pub fn solve_all_cases()
{
    run_cases(
        &["C-small-practice", "C-large-practice"],
        "y2008round1b",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let k: usize = reader.read_int();

                let nums = reader.read_num_line();
                assert_eq!(nums[0], nums.len() - 1);

                if case_no != 3 {
                    // continue;
                }

                println!("Solving case {}", case_no);

                writeln!(buffer, "Case #{}: {}", case_no,
                         solve(k, &nums[1..]).iter().join(" ")).unwrap();
            }
        },
    );
}

fn solve(k: usize, indices: &[usize]) -> Vec<usize>
{
    let mut bt = vec![0; k*k - 1];

    vec![1, 2]
}
