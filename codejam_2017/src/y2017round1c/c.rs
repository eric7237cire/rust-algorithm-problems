
//use std::cmp::min;
use std::cmp::Ordering::*;
use codejam::util::codejam::run_cases;
use std::io::Write;

/*
hard custom algorithm / proof
dynamic programming
probability of success of at least K independent events
*/
pub fn solve_all_cases()
{
    run_cases(
        &["C-small-practice-1", "C-small-practice-2", ],
        "y2017round1c",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let (_, k) = reader.read_tuple_2::<u8>();
                let u = reader.read_int::<f64>();
                let mut p = reader.read_num_line::<f64>();

                println!("Solving case {}", case_no);

                writeln!(buffer, "Case #{}: {:.9}", case_no, solve(&mut p, u, k)).unwrap();
            }
        },
    );

}


fn prob_at_least_k(p: &[f64], k: usize) -> f64
{
    /*effectively this is a 2d matrix
    given P is a set of independent events probability success
    dyn[N][K] = the probability of at least K successes  from the first N events

    dyn[N][K] =
        //failure
        (1-P[N])*dyn[N-1][K]
        //success
        P[N] * dyn[N-1][K-1]

    Since we only need the previous row, we don't need to keep the entire matrix
    */
    let mut dp = vec![0f64; k + 1];

    //0 successful events = 100%
    dp[0] = 1f64;
    for p in p {
        let prev = dp.clone();
        for k in 1..=k {
            dp[k] = (1f64 - p) * prev[k] + p * prev[k - 1];
        }
    }
    return dp[k];
}

fn fmin(a: f64, b: f64) -> f64
{
    if a.partial_cmp(&b).unwrap() == Greater {
        b
    } else {
        a
    }
}

#[test]
fn prob1()
{
    let pp = [0.5; 6];
    let p = prob_at_least_k(&pp, 3);
    println!("Prob 3 heads of 6 coins: {:.5}", p);
    let pp = [0.5; 18];
    let p = prob_at_least_k(&pp, 12);
    println!("Prob 12 heads of 18 coins: {:.5}", p);
}

fn solve(prob: &mut Vec<f64>, u: f64, k: u8) -> f64
{
    prob.sort_by(|&a, &b| a.partial_cmp(&b).unwrap());

    let mut best_ans = -1f64;
    for i in 0..prob.len() {
        let mut p_improved: Vec<_> = prob.clone();
        let mut u_remaining = u;

        //distribute u to lowest
        for j in i..prob.len() {
            let number_improving = j - i + 1;
            let next_p = if j == prob.len() - 1 {
                1f64
            } else {
                p_improved[j + 1]
            };
            let improvement_amount = fmin(
                next_p - p_improved[j],
                u_remaining / number_improving as f64,
            );
            for jj in i..j + 1 {
                p_improved[jj] += improvement_amount;
                u_remaining -= improvement_amount;
            }

            if u_remaining < 0f64 {
                break;
            }
            /*
            debug!(
                "After j={}, P={:?}, U={}, U remaning={}",
                j, p_improved, U, u_remaining
            );*/
        }

        //now distribute to i-1 if we have any left
        if i > 0 {
            let possible_improvement_to_i_minus_1 = fmin(u_remaining, 1f64 - p_improved[i - 1]);

            p_improved[i - 1] += possible_improvement_to_i_minus_1;
            u_remaining -= possible_improvement_to_i_minus_1;

            //we should have found the optimal answer
            if u_remaining > 0f64 {
                debug!("U remaning {}, breaking", u_remaining);
                break;
            }
        }

        let at_least_k = prob_at_least_k(&p_improved, k.into());

        debug!(
            "i={} probablity after distributing U {} = {:?}.  Overall success prob={} for K {}",
            i, u, p_improved, at_least_k, k
        );

        if at_least_k > best_ans {
            best_ans = at_least_k;
        }
    }

    best_ans
}
