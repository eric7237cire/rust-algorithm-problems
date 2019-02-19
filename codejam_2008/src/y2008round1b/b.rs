use codejam::algo::graph::disjointset::DisjointSet;
use codejam::util::codejam::run_cases;

use std::io::Write;
use primal::Sieve;

/*
Disjoint set
Primes sieve
*/
pub fn solve_all_cases()
{
    let sieve = Sieve::new(1_000_001);

    run_cases(
        &["B-small-practice", "B-large-practice"],
        "y2008round1b",
        |reader, buffer| {
            let t = reader.read_int();


            for case_no in 1..=t {
                let (a, b, p) = reader.read_tuple_3();

                if case_no != 3 {
                    // continue;
                }

                println!("Solving case {}", case_no);

                writeln!(
                    buffer,
                    "Case #{}: {}",
                    case_no,
                    solve(a, b, p, &sieve)
                )
                .unwrap();
            }
        },
    );
}

fn solve(a: u64, b: u64, p_lower_bound: u64, sieve: &Sieve) -> usize
{
    //First step, find  P <= primes <= interval
    let lower_bound_primes = p_lower_bound;
    let upper_bound_primes = b - a;

    let interval_size = (b - a + 1) as usize;

    let mut ds = DisjointSet::new(interval_size);

    for prime in sieve.primes_from(p_lower_bound as usize).take_while(|x| *x <= upper_bound_primes as usize) {
        let prime = prime as u64;
        let a_mod_prime = a % prime;
        let interval_p = if a_mod_prime == 0 {
            a
        } else {
            a + prime - a_mod_prime
        };

        //printf("First hit in interval %lld\n", interval_p);

        let set_to_merge = interval_p - a;

        for i in (interval_p - a + prime..interval_size as u64).step_by(prime as usize) {
            ds.merge_sets(i as usize, set_to_merge as usize);
        }
    }
    ds.number_of_sets()
}
