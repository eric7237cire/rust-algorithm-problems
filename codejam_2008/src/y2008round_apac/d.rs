use codejam::util::codejam::run_cases;
use std::io::Write;

/*
TODO
*/
pub fn solve_all_cases()
{
    run_cases(
        &["D-small-practice", "D-large-practice"],
        "y2008round_apac",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                if case_no != 21 {
                    // continue;
                }
                println!("Solving case {}", case_no);

                writeln!(buffer, "Case #{}: {}", case_no, solve()).unwrap();
            }
        },
    );
}

fn solve() -> isize
{
    3
}
