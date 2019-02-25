use codejam::util::bitvec64::BitVec64;
use codejam::util::codejam::run_cases;
use codejam::util::vector_2d::Vector2d;
use num_bigint::BigUint;
//use num_integer::binomial;
use std::io::Write;
use std::usize;

/*
Linear change of basis
Modular multiplicative inverse
binary exponentiation algorithm
Fermat's little theorem
Lattice walk
Lucas's theorem
*/
pub fn solve_all_cases()
{
    let fac = compute_modular_factorial(MODULUS);
    let inv = compute_modular_inverse(MODULUS);

    run_cases(
        &["D-small-practice",
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
            if rock.r() > target.r() || rock.c() > target.c() {
                continue;
            }
            rocks.push(rock);
        }
    }

    rocks.sort_by(|a, b| a.c().cmp(&b.c()).then_with(|| a.r().cmp(&b.r())));

    rocks.insert(0, start);
    rocks.push(target);

    debug!("Rocks {:?}", rocks);

    let mut ans = 0;
    //always include start & target, so subset -2
    for subset in 0..1 << (rocks.len() - 2) {
        let bs = BitVec64::with_val(subset);
        let sign = if bs.pop_count() % 2 == 0 { 1 } else { -1 };

        let mut last = 0;

        let ways: usize = (1..rocks.len())
            .filter(|&idx| idx == 0 || idx == rocks.len() - 1 || bs.get(idx - 1))
            .map(|cur| {
                if rocks[last].r() > rocks[cur].r() {
                    last = cur;
                    return 0;
                }
                assert!(cur != last);
                assert!(rocks[cur].c() >= rocks[last].c(),
                        "cur {} last {} {:?} {:?}", cur, last, rocks[cur],
                rocks[last]);

                let m = rocks[cur].r() - rocks[last].r();
                let n = rocks[cur].c() - rocks[last].c();
                last = cur;
                n_choose_k_mod(m + n, n, MODULUS, fac, inv)
            })
            .fold(1, |acc, w| (acc * w) % MODULUS);

        debug!("Rocks {:0>width$b} ways {} sign {}", bs.data, ways, sign, width=rocks.len());
        ans += 10007 + sign * ways as isize;
    }
    ans % 10007
}

//Uses lucas theorum and modular inversese
fn n_choose_k_mod(
    n: usize,
    k: usize,
    p: usize,
    fac: &[usize],
    inv: &[usize]
) -> usize
{
    let mut n = n;
    let mut k = k;

    let mut product = 1;
    while k > 0 {
        let key = (n % p, k % p);


        if key.1 > key.0 {
            product *= 0;
        }
        else if key.1 == 0 || key.0 == key.1 {
            product *= 1;
        }
        else {
            //n! / (k! * (n-k)!)
            let n_fact = fac[n % p];
            let k_fact = inv[fac[k % p]];
            let n_sub_k_fact = inv[fac[neg_mod(n,k,p)]];

            let pp = (n_fact * k_fact * n_sub_k_fact) % p;
            product *= pp;

/*
            let b = biguint_to_usize(
                &(binomial(BigUint::from(key.0), BigUint::from(key.1)) % BigUint::from(p)),
            );

            assert_eq!(b, pp, "{} choose {}", key.0, key.1);*/
        }

        product %= p;

        n /= p;
        k /= p;
    }

    product
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

#[allow(dead_code)]
fn biguint_to_usize(bu: &BigUint) -> usize
{
    let mut ans = 0usize;

    for (i, n) in bu.to_radix_le(256).into_iter().enumerate() {
        ans |= (n as usize) << (i * 8);
    }

    ans
}

//https://cp-algorithms.com/algebra/module-inverse.html
///Modular inverse 1..m
fn compute_modular_inverse(m: usize) -> Vec<usize>
{
    //https://cp-algorithms.com/algebra/module-inverse.html
    let mut inv = vec![0; m ];

   // inv[0] = 1;
    inv[1] = 1;
    for i in 2..m {
        inv[i] = (m - (m / i) * inv[m % i] % m) % m;
    }
    inv
}

///(1..m)
fn compute_modular_factorial(m: usize) -> Vec<usize>
{
    //https://cp-algorithms.com/algebra/module-inverse.html
    let mut fac = vec![0; m + 1];

    fac[1] = 1;
    for i in 2..m {
        fac[i] = (fac[i - 1] * i) % m;
    }
    fac
}

//https://cp-algorithms.com/algebra/module-inverse.html
fn compute_modular_inverse_1(a: usize, modulus: usize) -> usize
{
    binpow(a, modulus - 2, modulus)
}

///Binary power a^b % modulus
/// Modulus must be prime
#[allow(dead_code)]
fn binpow(a: usize, b: usize, modulus: usize) -> usize
{
    let mut a = a % modulus;
    let mut b = b;
    let mut res = 1;
    while b > 0 {
        if b & 1 > 0 {
            res = (res * a) % modulus;
        }
        a = (a * a) % modulus;
        b >>= 1;
    }
    return res;
}

//https://stackoverflow.com/questions/9727962/fast-way-to-calculate-n-mod-m-where-m-is-prime
#[allow(dead_code)]
fn factorial_mod(n: usize, modulus: usize) -> usize
{
    let mut ans: isize = 1;
    if n <= modulus / 2 {
        //#calculate the factorial normally (right argument of range() is exclusive)
        for i in 1..=n {
            ans = (ans * i as isize) % modulus as isize;
        }
    } else if n >= modulus {
        //because 1*2*...*modulus*...*n === 0
        return 0;
    } else {
        //Fancypants method for large n
        for i in n + 1..modulus {
            ans = (ans * i as isize) % modulus as isize;
        }
        ans = compute_modular_inverse_1(ans as usize, modulus) as isize;
        ans = -1 * ans + modulus as isize;
    }
    return (ans % modulus as isize) as usize;
}

/// (a-b) % m
fn neg_mod(a: usize, b: usize, m: usize) -> usize
{
    let mut a_sub_b = a as isize - b as isize;
    a_sub_b %= m as isize;
    if a_sub_b < 0 {
        a_sub_b += m as isize;
    }
    a_sub_b as usize
}

#[cfg(test)]
mod test_endless_knight
{
    use super::*;
    use num_integer::binomial;

    #[test]
    fn test_n_choose_k()
    {
        let modulus = 79 ; //1033; //10007;
        let fac = compute_modular_factorial(modulus);
        let inv = compute_modular_inverse(modulus);
        let p = BigUint::from(modulus);

        //let check_fac

        for n in 1..modulus {

            for k in 2..n {
                println!("N= {} K={}", n,k);
                let check = biguint_to_usize(&(binomial(BigUint::from(n),
                                                        BigUint::from(k)) % &p));

                let n_fact = fac[n ];
                let k_fact = inv[ fac[k] ];
                let n_sub_k_fact = inv[ fac[n - k] ];

                assert_eq!(n_fact, factorial_mod(n, modulus));
                assert_eq!(k_fact, compute_modular_inverse_1(factorial_mod(k, modulus),modulus ), "k_fact wrong");
                assert_eq!(n_sub_k_fact, compute_modular_inverse_1(factorial_mod(n-k, modulus),modulus ));

                assert_eq!((n_fact * k_fact * n_sub_k_fact) % modulus, check);
            }
        }
    }

    #[test]
    fn test_factorial_mod()
    {
        for &prime in [7, 10007].iter() {
            let pu = BigUint::from(prime);
            let mut f = BigUint::from(1usize);

            for i in 2..=23 {
                f *= BigUint::from(i);
                assert_eq!(biguint_to_usize(&(&f % &pu)), factorial_mod(i, prime));
            }
        }
    }

    #[test]
    fn test_modular_inverse_2()
    {
        let prime = 10007;
        let inverse = compute_modular_inverse(prime);

        for i in 1..prime {

            assert_eq!(inverse[i], compute_modular_inverse_1(i, prime));
        }
    }

    #[test]
    fn test_modular_inverse()
    {
        let prime = 23;
        let inverse = compute_modular_inverse(prime);

        let numerator = [2, 2, 2, 2, 3, 3, 5, 7, 11, 13, 17, 19, 23]
            .iter()
            .fold(1, |acc, n| acc * *n);

        for i in 2..23 {
            //just make sure numerator is divisible by i
            assert_eq!(0, numerator % i, "{}", i);
            let check = (numerator / i) % prime;

            //check numerator / i === inverse[i] * numerator
            assert_eq!(check, ((numerator % prime) * inverse[i]) % prime);

            assert_eq!(inverse[i], compute_modular_inverse_1(i, prime));
        }
    }

    #[test]
    fn test_neg_mod()
    {
        assert_eq!(-7 % 3, -1);
    }

    #[test]
    fn test_binomial()
    {
        assert_eq!(0, binomial(2, 15));
    }

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
            //assert_eq!(product % p, n_choose_k_mod(n, k, *p));
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
