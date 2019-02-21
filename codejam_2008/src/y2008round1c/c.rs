use codejam::util::binary_sum_tree::BinarySumTree;
use codejam::util::codejam::run_cases;
use std::io::Write;
use superslice::Ext;

/*
Binary Sum tree using an array
*/
pub fn solve_all_cases()
{
    run_cases(
        &["C-small-practice", "C-large-practice"],
        "y2008round1c",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                /*
                The first line of each case contains n, m, X, Y and Z
                each separated by a space. n will be the length of the sequence
                of speed limits. m will be the length of the generating array A.
                The next m lines will contain the m elements of A,
                one integer per line (from A[0] to A[m-1]).
                */
                let num_line: Vec<u64> = reader.read_num_line();
                assert_eq!(5, num_line.len());
                let m = num_line[1] as usize;

                /*
                              for i = 0 to n-1
                print A[i mod m]
                A[i mod m] = (X * A[i mod m] + Y * (i + 1)) mod Z
                */

                let mut a: Vec<u64> = (0..m).map(|_| reader.read_int()).collect();
                let n = num_line[0] as usize;
                let x = num_line[2];
                let y = num_line[3];
                let z = num_line[4];

                let mut speed_limits = (0..n)
                    .map(|i| {
                        let next = a[i % m];
                        a[i % m] = (x * a[i % m] + y * (i + 1) as u64) % z;
                        next
                    })
                    .collect();

                if case_no != 3 {
                    // continue;
                }

                println!("Solving case {}", case_no);

                writeln!(buffer, "Case #{}: {}", case_no, solve(&mut speed_limits)).unwrap();
            }
        },
    );
}

const MOD: u64 = 1_000_000_007;
fn solve(speed_limits: &mut Vec<u64>) -> u64
{
    debug!("Speed limits: {:?}", speed_limits);
    let mut sorted_speed_limits = speed_limits.clone();
    sorted_speed_limits.sort();

    //normalize everything
    for si in speed_limits.iter_mut() {
        *si = sorted_speed_limits.lower_bound(si) as u64;
    }

    //offset by 1 so bt[1] == # of sequences ending with 0
    let mut bt = BinarySumTree::new(speed_limits.len() + 1);

    for speed_limit in speed_limits.iter() {
        let sl = *speed_limit as usize;
        //We have a new subsequence for every existing sequence that
        // ends in a value < speedLimit
        let num_seq = (1 + bt.sum_to(sl)) % MOD;

        bt.set(sl, num_seq);
    }

    bt.sum() % MOD
}
