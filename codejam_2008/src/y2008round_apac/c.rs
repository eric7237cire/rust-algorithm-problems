use bit_vec::BitVec;
use codejam::util::codejam::run_cases;
use codejam::util::vector_2d::Vector2d;
use std::io::Write;
use std::usize;

/*
TODO
*/
pub fn solve_all_cases()
{
    run_cases(
        &["C-small-practice", "C-large-practice"],
        "y2008round3",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let (n_rows, n_cols) = reader.read_tuple_2();

                let mut chairs = BitVec::from_elem(n_cols * n_rows, false);

                for r in 0..n_rows {
                    for (c, ch) in reader.read_chars(n_cols).into_iter().enumerate() {
                        let idx = r * n_cols + c;
                        chairs.set(idx, ch == '.');
                    }
                }

                if case_no != 3 {
                    //continue;
                }
                println!("Solving case {}", case_no);

                writeln!(
                    buffer,
                    "Case #{}: {}",
                    case_no,
                    solve(&chairs, n_rows, n_cols)
                )
                .unwrap();
            }
        },
    );
}

const INVALID: usize = usize::MAX;

fn index_to_vec(idx: usize, n_cols: usize) -> Vector2d<isize>
{
    Vector2d::with_val((idx / n_cols) as isize, (idx % n_cols) as isize)
}
fn vec_to_index(v: &Vector2d<isize>, n_cols: usize) -> usize
{
    v.r() as usize * n_cols + v.c() as usize
}
fn vec_comp_to_index(r: isize, c: isize, n_cols: usize) -> usize
{
    r as usize * n_cols + c as usize
}

fn solve(chairs: &BitVec, n_rows: usize, n_cols: usize) -> usize
{
    3
}
