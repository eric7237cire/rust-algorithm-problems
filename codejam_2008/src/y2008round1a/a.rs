use codejam::util::codejam::run_cases;
use std::io::Write;


/*
Arithmetic
Proof?
*/
pub fn solve_all_cases()
{
    run_cases(
        &["A-small-practice", "A-large-practice"],
        "y2008round1a",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let _n : usize = reader.read_int();


                let mut v1 = reader.read_num_line();
                let mut v2 = reader.read_num_line();

                if case_no != 3 {
                    // continue;
                }

                println!("Solving case {}", case_no);

                writeln!(
                    buffer,
                    "Case #{}: {}",
                    case_no,
                    solve(
                        &mut v1, &mut v2
                    )
                )
                .unwrap();
            }
        },
    );
}

fn solve(v1: &mut Vec<i64>, v2: &mut Vec<i64>) -> i64
{
    v1.sort();
    v2.sort();
    v2.reverse();

    v1.iter().zip(v2.iter()).map( |(e1,e2)| e1 * e2).sum()



}
