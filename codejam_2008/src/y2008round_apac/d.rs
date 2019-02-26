use codejam::util::bitvec64::BitVec64;
use codejam::util::codejam::run_cases;
use codejam::util::vector_2d::Vector2d;
use num_bigint::BigUint;
//use num_integer::binomial;
use std::io::Write;
use std::usize;

/*
TODO
*/
pub fn solve_all_cases()
{
    let fac = compute_modular_factorial(MODULUS);
    let inv = compute_modular_inverse(MODULUS);

    run_cases(
        &["D-small-practice", "D-large-practice"],
        "y2008round3",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let (n_rows, n_cols, n_rocks) = reader.read_tuple_3();

                let rocks: Vec<Vector2d<isize>> = (0..n_rocks)
                    .map(|_| {
                        let (r, c): (isize, isize) = reader.read_tuple_2();
                        Vector2d::with_val(r - 1, c - 1)
                    })
                    .collect();

                if case_no != 21 {
                    // continue;
                }
                println!("Solving case {}", case_no);

                writeln!(
                    buffer,
                    "Case #{}: {}",
                    case_no,
                    solve(rocks.as_slice(), n_rows, n_cols, &fac, &inv)
                )
                .unwrap();
            }
        },
    );
}

const MODULUS: usize = 10007;

fn solve(
    rocks_orig: &[Vector2d<isize>],
    n_rows: isize,
    n_cols: isize,
    fac: &[usize],
    inv: &[usize],
) -> isize
{
3
}
