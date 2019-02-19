use codejam::algo::graph::disjointset::DisjointSet;
use codejam::algo::prime::sieve::SieveOfAtkin;
use codejam::util::codejam::run_cases;

use std::io::Write;

/*
Disjoint set
Primes sieve
*/
pub fn solve_all_cases()
{
    run_cases(
        &["B-small-practice", "B-large-practice"],
        "y2008round1b",
        |reader, buffer| {
            let t = reader.read_int();

            let mut sieve = SieveOfAtkin::new(100_000);
            println!("Starting sieve");
            sieve.run();
            let primes: Vec<u64> = sieve.get_results_vec();
            println!("Finished sieve");

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
                    solve(a, b, p, primes.as_slice())
                )
                .unwrap();
            }
        },
    );
}

fn solve(a: u64, b: u64, p_lower_bound: u64, primes: &[u64]) -> usize
{
    //First step, find  P <= primes <= interval
    let lower_bound_primes = p_lower_bound;
    let upper_bound_primes = b - a;

    let interval_size = (b - a + 1) as usize;

    let mut ds = DisjointSet::new(interval_size);

    for prime in primes.iter().filter(|p| **p >= p_lower_bound) {
        let a_mod_prime = a % prime;
        let interval_p = if a_mod_prime == 0 {
            a
        } else {
            a + prime - a_mod_prime
        };

        //printf("First hit in interval %lld\n", interval_p);

        let set_to_merge = interval_p - a;

        for i in (interval_p - a + prime..interval_size as u64).step_by(*prime as usize) {
            ds.merge_sets(i as usize, set_to_merge as usize);
        }
    }
    ds.number_of_sets()
}
