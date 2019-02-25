use codejam::util::bitvec64::BitVec64;
use codejam::util::codejam::run_cases;
use codejam::util::vector_2d::Vector2d;
use num_bigint::BigUint;
use num_integer::binomial;
use num_traits::identities::Zero;
use std::io::Write;
use std::usize;

/*
Change of base
*/
pub fn solve_all_cases()
{
    run_cases(
        &[
            "D-small-practice",
                "D-large-practice"
        ],
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

                if case_no != 40 {
                    //continue;
                }
                println!("Solving case {}", case_no);

                writeln!(
                    buffer,
                    "Case #{}: {}",
                    case_no,
                    solve(rocks.as_slice(), n_rows, n_cols)
                )
                .unwrap();
            }
        },
    );
}

fn solve(rocks_orig: &[Vector2d<isize>], n_rows: isize, n_cols: isize) -> isize
{
    let target = change_basis(&Vector2d::with_val(n_rows - 1, n_cols - 1));

    debug!("N rows {} cols {} target {:?}", n_rows, n_cols, target);

    if target.is_none() {
        return 0;
    }

    let target = target.unwrap();
    let start = Vector2d::with_val(0, 0);

    let mut rocks = Vec::new();

    for r in rocks_orig.iter() {
        if let Some(rock) = change_basis(r) {
            //exceeds destination
            if rock.r() >= n_rows as usize || rock.c() >= n_cols as usize {
                continue;
            }
            rocks.push(rock);
        }
    }

    let mut ans = 0;
    for subset in 0..1 << rocks.len() {
        let bs = BitVec64::with_val(subset);
        let sign = if bs.pop_count() % 2 == 0 { 1 } else { -1 };

        let mut rocks_subset: Vec<Vector2d<usize>> = rocks
            .iter()
            .enumerate()
            .filter(|(idx, _)| bs.get(*idx))
            .map(|(_, rock)| *rock)
            .collect();

        //rocks_subset.push(start);
        //rocks_subset.push(target);

        rocks_subset.sort_by(|a, b|
            a.c().cmp(&b.c()).then_with(|| a.r().cmp(&b.r())));

        rocks_subset.insert(0, start);
        rocks_subset.push(target);

        let ways: usize = rocks_subset
            .windows(2)
            .map(|win| {
                if win[0].r() > win[1].r() {
                    return 0;
                }
                let m = win[1].r() - win[0].r();
                let n = win[1].c() - win[0].c();
                n_choose_k_mod(m + n, n, 10007)
            })
            .fold(1, |acc, w| (acc * w) % 10007);

        debug!("Rocks {:?} ways {} sign {}", rocks_subset, ways, sign);
        ans += 10007 + sign * ways as isize;
    }
    ans % 10007
}

fn n_choose_k_mod(n: usize, k: usize, p: usize) -> usize
{
    let mut n_i = BigUint::from(n);
    let mut k_i = BigUint::from(k);

    let mut product = BigUint::from(1usize);
    let p = BigUint::from(p);
    while k_i > BigUint::zero() {
        product *= binomial(&n_i % &p, &k_i % &p);

        n_i /= &p;
        k_i /= &p;
    }

    biguint_to_usize(&(product % p))
}

fn change_basis(rc: &Vector2d<isize>) -> Option<Vector2d<usize>>
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

    if r % 3 != 0 || c % 3 != 0 || r < 0 || c < 0 {
        None
    } else {
        Some(Vector2d::with_val(r as usize / 3, c as usize / 3))
    }
}

fn biguint_to_usize(bu: &BigUint) -> usize
{
    let mut ans = 0usize;

    for (i, n) in bu.to_radix_le(256).into_iter().enumerate() {
        ans |= (n as usize) << (i * 8);
    }

    ans
}

#[cfg(test)]
mod test_endless_knight
{
    use super::*;
    use num_integer::binomial;

    #[test]
    fn test_biguint_to_usize()
    {
        let n = 987_654_123_456_987;
        let bu = BigUint::from(n);
        assert_eq!(n, biguint_to_usize(&bu));
    }

    #[test]
    fn test_lucas_theorum()
    {
        let n = 38;
        let k = 8;
        let n_choose_k = binomial(n, k);

        debug!("{} choose {} = {}", n, k, n_choose_k);

        for p in [7, 11, 17, 23, 47, 127].iter() {
            let mut n_i = n;
            let mut k_i = k;

            let mut product = 1;
            while k_i > 0 {
                product *= binomial(n_i % p, k_i % p);

                n_i /= p;
                k_i /= p;
            }

            debug!(
                "Product = {} n_choose_k % {} = {}",
                product,
                p,
                n_choose_k % p
            );
            assert_eq!(product % p, n_choose_k % p);
            assert_eq!(product % p, n_choose_k_mod(n, k, *p));
        }
    }

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

        assert_eq!(None, change_basis(&Vector2d::with_val(3, 1)));

        let vec1 = Vector2d::with_val(2, 1);
        let vec2 = Vector2d::with_val(1, 2);
        for r in 0..100usize {
            for c in 0..100usize {
                let v = vec1 * r + vec2 * c;
                assert_eq!(Some(Vector2d::with_val(r, c)), change_basis(&v.convert()));
                // println!("v is {:?}", v);
            }
        }
    }
}
