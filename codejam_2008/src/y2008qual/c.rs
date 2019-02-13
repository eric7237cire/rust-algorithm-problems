//use bit_set::BitSet;
use codejam::util::codejam::run_cases;
use std::io::Write;
use std::usize;
use std::cmp::min;
use std::cmp::Ordering;

/*

*/
pub fn solve_all_cases()
{
    run_cases(
        &["C-small-practice", "C-large-practice"],
        "y2008qual",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {

                let floats = reader.read_num_line();



                if case_no != 1 {
                    // continue;
                }

                println!("Solving case {}", case_no);

                let ans = solve(floats[0], floats[1], floats[2], floats[3], floats[4]);
                writeln!(
                    buffer,
                    "Case #{}: {}",
                    case_no, ans

                )
                .unwrap();
            }
        },
    );
}

fn solve(fly_radius: f64, racket_radius : f64, t: f64, chord_radius: f64, gap_len:f64) -> f64
{
   0f64
}
