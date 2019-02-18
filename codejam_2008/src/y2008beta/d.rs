use codejam::util::codejam::run_cases;
use std::io::Write;



/*
*/
pub fn solve_all_cases()
{
    run_cases(
        &["D-small-practice", "D-large-practice"],
        "y2008beta",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let positions = reader.read_num_line();
                let values = reader.read_num_line();

                if case_no != 3 {
                    // continue;
                }

                println!("Solving case {}", case_no);

                writeln!(
                    buffer,
                    "Case #{}: {}",
                    case_no,
                    solve(
                        positions.as_slice(),
                        values.as_slice()
                    )
                )
                .unwrap();
            }
        },
    );
}

fn solve(positions: &[u16], values: &[u16]) -> u16
{

    47
}
