use codejam::util::codejam::run_cases;
use codejam::util::grid::constants::*;
use codejam::util::vector_2d::Vector2d;
use itertools::Itertools;
use std::cmp::max;
use std::cmp::min;
use std::io::Write;
use std::isize;

/*

*/
pub fn solve_all_cases()
{
    run_cases(
        &["A-small-practice", "A-large-practice"],
        "y2008round_apac",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let n = reader.read_int();

                //h w, is bird
                let birds: Vec< (Vector2d<isize>, bool) > = (0..n).map( |_| {
                    let s = reader.read_string_line();
                    (Vector2d::with_val( s[0].parse().unwrap(), s[1].parse().unwrap()), s[2] == "BIRD")
                }).collect();

                let m:usize = reader.read_int();

                let unknown: Vec<Vector2d<isize>> = (0..n).map( |_| {
                    let (h,w) = reader.read_tuple_2();
                    Vector2d::with_val(h,w)
                }).collect();

                if case_no != 3 {
                    // continue;
                }

                println!("Solving case {}", case_no);

                writeln!(buffer, "Case #{}: {}", case_no, solve(&birds, &unknown)).unwrap();
            }
        },
    );
}
fn solve(birds: &[(Vector2d<isize>, bool)], unknown: &[Vector2d<isize>] ) -> isize
{
    3
}
