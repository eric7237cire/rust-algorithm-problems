use bit_vec::BitVec;
use codejam::util::codejam::run_cases;
use codejam::util::vector_2d::Vector2d;
use std::io::Write;
use std::usize;

/*
Change of base
*/
pub fn solve_all_cases()
{
    run_cases(
        &["D-small-practice", "D-large-practice"],
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

fn solve(chairs: &BitVec, n_rows: usize, n_cols: usize) -> usize
{
    3
}

fn change_basis(rc: &Vector2d<isize>) -> Option<Vector2d<isize>>
{
    /*
    https://en.wikipedia.org/wiki/Change_of_basis

    take a matrix
    [ 2 1
      1 2 ] find the inverse

     [ 2/3 -1/3
      -1/3 2/3 ]

      r' = 2/3 r - 1/3 c
      c' = -1/3 r + 2/3 c

    */

    let r = 2 * rc.r() - rc.c();
    let c = 2 * rc.c() - rc.r();

    if r % 3 != 0 || c % 3 != 0 {
        None
    } else {
        Some(Vector2d::with_val(r / 3, c / 3))
    }
}

#[cfg(test)]
mod test_endless_knight
{
    use super::*;

    //cargo test test_edge_iterator -- --nocapture
    #[test]
    fn test_change_basis()
    {
        assert_eq!(
            Some(Vector2d::with_val(1, 0)),
            change_basis(&Vector2d::with_val(2, 1))
        );

        assert_eq!(
            Some(Vector2d::with_val(3, 2)),
            change_basis(&Vector2d::with_val(8, 7))
        );

        assert_eq!(
            Some(Vector2d::with_val(1, 1)),
            change_basis(&Vector2d::with_val(3, 3))
        );

        assert_eq!(
            None,
            change_basis(&Vector2d::with_val(3, 1))
        );

        let vec1 = Vector2d::with_val(2, 1);
        let vec2 = Vector2d::with_val(1, 2);
        for r in 0..100 {
            for c in 0..100 {
                let v = vec1 * r + vec2 * c;
                assert_eq!(Some(Vector2d::with_val(r, c)), change_basis(&v));
                // println!("v is {:?}", v);
            }
        }
    }
}
