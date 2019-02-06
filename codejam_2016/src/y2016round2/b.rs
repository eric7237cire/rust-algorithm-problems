use codejam::util::codejam::run_cases;
use std::cmp::max;
use itertools::Itertools;
use std::io::Write;
use std::mem::swap;

use permutohedron::LexicalPermutation;

/*

*/
pub fn solve_all_cases()
{
    run_cases(
        &["B-small-practice",
         //"A-large-practice"
        ],
        "y2016round2",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                

                let (N,K) = reader.read_tuple_2();
                
                let prob = reader.read_num_line::<f64>();

                assert_eq!(N, prob.len());

                if case_no > 3 {
                    // continue;
                }

                //debug!("Solving case {}", case_no);

                writeln!(buffer, "Case #{}: {}", case_no, 
                //solve(K, &prob)
                solve_brute_force(K, &prob)
                ).unwrap();
            }
        },
    );
}

fn f64_max(a: f64, b: f64) -> f64 
{
    if a >= b {
        a
    } else {
        b 
    }

}

fn solve_brute_force(K: usize, prob_list: &[f64]) -> f64
{
    let mut perm : Vec<usize> = (0..prob_list.len()).map( |p| if p < K {1} else {0}).collect();

    perm.sort();

    

    let mut best_p = 0.;

    loop {

        let subset : Vec<f64> = prob_list.iter().enumerate()
        .filter( |(idx, _)| perm[*idx] == 1 )
        .map(| (_,p)| *p).collect();
        debug!("Checking subset {:?}", subset);

        assert_eq!(subset.len(), K);

        let mut dp = vec![0.; 1+K ];
        let mut next_dp = vec![0.; 1+K ];

        dp[0] = 1.;
        
        for (idx, &p) in subset.iter().enumerate()
        {
            next_dp[0] = dp[0] * (1.-p);

            for k in 1..=K 
            {
                next_dp[k] = dp[k-1] * p + dp[k] * (1.-p);
            }

            debug!("After {} in subset {:?}, probabilites are: {:?}.  Prob of k/2 {} = {}", 
            idx, subset,
            dp, K/2, dp[K/2]);

            swap(&mut dp, &mut next_dp);
        }

        debug!("For subset, probabilites are: {:?}.  Prob of k/2 {} = {}", dp, K/2, dp[K/2]);

        if dp[K/2] >= best_p {
            debug!("new best");
            best_p = dp[K/2];
        }

        if (!perm.next_permutation()) {
            break;
        }
    }

    best_p 

}


fn solve(K: usize, prob_list: &[f64]) -> f64
{
    debug!("Solving K={}  prob={:?} ", K, prob_list);

    assert!(prob_list.len() <= 200);

    let dp_ubound = prob_list.len() + 1;

    let mut dp = vec![ vec![ vec![ 0f64; dp_ubound] ; dp_ubound] ; dp_ubound ];

    dp[0][0][0]  = 1.;

    //dp[ using guys up to][used # of guys] [ # of yes votes ]
    for (p_idx, prob) in prob_list.iter().enumerate() 
    {
        //either guy is not in K
        for used in 0..dp_ubound {
            dp [ p_idx + 1 ][used] = dp [p_idx][used].clone();
        }

        //can anything be improved
        for used in 1..=1+p_idx {
            for yes in 1..=1+p_idx {
                debug!("Calcing {} used {} yes {} prob {} dp {}", p_idx, used, yes, prob, dp[p_idx][used-1][yes-1]);
                dp[p_idx+1][used][yes] = f64_max( f64_max( dp[p_idx][used][yes], prob * dp[p_idx][used-1][yes-1]), (1.-prob) * dp[p_idx][used-1][yes]);

            }
        }
    }

    for p in 1..dp_ubound {
        for used in 0..dp_ubound {
            for yes in 0..dp_ubound {
                debug!("Using people up to {}, {} people, {} exact yes votes = {}", 
                p, used, yes, dp[p][used][yes]
                )
            }
        }
    }

    dp[prob_list.len()][K][K/2]
}