use codejam::util::codejam::run_cases;
//use std::cmp::max;
//use itertools::Itertools;
use std::io::Write;
use std::mem::swap;

use permutohedron::LexicalPermutation;

/*
Probability of exactly K events
Dynamic Programming
Maximizing probability
*/
pub fn solve_all_cases()
{
    run_cases(
        &["B-small-practice", "B-large-practice"],
        "y2016round2",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let (N, K) = reader.read_tuple_2();

                let prob = reader.read_num_line::<f64>();

                assert_eq!(N, prob.len());

                if case_no > 3 {
                    // continue;
                }

                println!("Solving case {}", case_no);

                writeln!(
                    buffer,
                    "Case #{}: {:.8}",
                    case_no,
                    solve(K, &prob) //solve_brute_force(K, &prob)
                )
                .unwrap();
            }
        },
    );
}

#[allow(dead_code)]
fn solve_brute_force(K: usize, prob_list: &[f64]) -> f64
{
    let mut prob_list = prob_list.to_vec();

    prob_list.sort_by(|a, b| a.partial_cmp(b).unwrap());

    println!("Prob list {:?}", prob_list);

    let mut perm: Vec<usize> = (0..prob_list.len())
        .map(|p| if p < K { 1 } else { 0 })
        .collect();

    perm.sort();

    let mut best_p = 0.;

    loop {
        let subset: Vec<f64> = prob_list
            .iter()
            .enumerate()
            .filter(|(idx, _)| perm[*idx] == 1)
            .map(|(_, p)| *p)
            .collect();
        debug!("Checking subset {:?}", subset);

        assert_eq!(subset.len(), K);

        let mut dp = vec![0.; 1 + K];
        let mut next_dp = vec![0.; 1 + K];

        dp[0] = 1.;

        for (idx, &p) in subset.iter().enumerate() {
            next_dp[0] = dp[0] * (1. - p);

            for k in 1..=K {
                next_dp[k] = dp[k - 1] * p + dp[k] * (1. - p);
            }

            debug!(
                "After {} in subset {:?}, probabilites are: {:?}.  Prob of k/2 {} = {}",
                idx,
                subset,
                dp,
                K / 2,
                dp[K / 2]
            );

            swap(&mut dp, &mut next_dp);
        }

        debug!(
            "For subset, probabilites are: {:?}.  Prob of k/2 {} = {}",
            dp,
            K / 2,
            dp[K / 2]
        );

        if dp[K / 2] > best_p {
            println!("new best.  subset: {:?}", perm);
            best_p = dp[K / 2];
        }

        if !perm.next_permutation() {
            break;
        }
    }

    best_p
}

fn solve(K: usize, prob_list: &[f64]) -> f64
{
    let mut prob_list = prob_list.to_vec();

    prob_list.sort_by(|a, b| a.partial_cmp(b).unwrap());

    debug!("Prob list {:?}", prob_list);

    for k in 0..K {
        prob_list.push(prob_list[k]);
    }

    let mut best_p = 0.;

    for subset in prob_list.windows(K) {
        debug!("Checking subset {:?}", subset);

        assert_eq!(subset.len(), K);

        let mut dp = vec![0.; 1 + K];
        let mut next_dp = vec![0.; 1 + K];

        dp[0] = 1.;

        for (idx, &p) in subset.iter().enumerate() {
            next_dp[0] = dp[0] * (1. - p);

            for k in 1..=idx + 1 {
                next_dp[k] = dp[k - 1] * p + dp[k] * (1. - p);
            }

            debug!(
                "After {} in subset {:?}, probabilites are: {:?}.  Prob of k/2 {} = {}",
                idx,
                subset,
                dp,
                K / 2,
                dp[K / 2]
            );

            swap(&mut dp, &mut next_dp);
        }

        debug!(
            "For subset, probabilites are: {:?}.  Prob of k/2 {} = {}",
            dp,
            K / 2,
            dp[K / 2]
        );

        if dp[K / 2] > best_p {
            debug!("new best.  subset: {:?}", subset);
            best_p = dp[K / 2];
        }
    }

    best_p
}
