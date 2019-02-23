
use std::cmp::min;
use std::io::Write;
use self::Parent::*;
use codejam::util::codejam::run_cases;

const DAY: usize = 24 * 60;

/*
dynamic programming
*/
#[derive(Clone, PartialEq)]
enum Parent
{
    Cameron,
    Jamie,
}

pub fn solve_all_cases()
{
    run_cases(
        &["B-small-practice", "B-large-practice"],
        "y2017round1c",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let (a_c, a_j) = reader.read_tuple_2::<u8>();

        let mut fixed: Vec<Option<Parent>> = vec![None; DAY];
        for i in 0..a_c + a_j {
            let (start, stop) = reader.read_tuple_2::<u16>();
            //intervals are open on right
            for t in start..stop {
                fixed[t as usize] = if i < a_c { Some(Cameron) } else { Some(Jamie) };
            }
        }
                println!("Solving case {}", case_no);

                writeln!(buffer, "Case #{}: {}", case_no, solve(&fixed)).unwrap();
            }
        },
    );

}

fn solve(fixed: &[Option<Parent>]) -> u16
{
    //From alkjash python solution

    /*
     Compute F[T][C][X] by dynamic programming, which is the minimal number of swaps needed for:
    # Deciding who does each of first T times
    # C <= T of that time goes to Cameron
    # X = 0 if Cameron does the last slot
    # X = 1 if Jamie does the last slot
    */
    let mut f = vec![[[(1 + DAY) as u16; 2]; 1 + DAY / 2]; 1 + DAY];
    //Stack overflow if we attempt to declare this on the stack
    //let mut F = [[ [(1+day) as u16;2] ; 1+day/2]; 1+day];

    f[0][0][0] = 0;
    f[0][0][1] = 0;
    for t in 1..=DAY {
        for c in 0..=DAY / 2 {
            if fixed[t - 1] != Some(Cameron) && c > 0 {
                // # Cameron is free for this minute
                //so cameron does this, either we need to switch from jamie or continue
                f[t][c][0] = min(f[t - 1][c - 1][1] + 1, f[t - 1][c - 1][0]);
            }
            if fixed[t - 1] != Some(Jamie) {
                // # Jamie is free for this minute
                f[t][c][1] = min(f[t - 1][c][1], f[t - 1][c][0] + 1);
            }
        }
    }

    let mut ans = min(f[DAY][DAY / 2][0], f[DAY][DAY / 2][1]);
    if ans % 2 == 1 {
        ans += 1;
    }

    ans
}
