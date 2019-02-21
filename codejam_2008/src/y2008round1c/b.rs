use codejam::util::codejam::run_cases;

use std::io::Write;

/*
Dynamic programming
Chinese remainder theorem

*/
pub fn solve_all_cases()
{
    run_cases(
        &["B-small-practice", "B-large-practice"],
        "y2008round1c",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let s: Vec<char> = reader.read_string().chars().collect();

                println!("Solving case {}", case_no);

                writeln!(buffer, "Case #{}: {}", case_no, solve(&s)).unwrap();
            }
        },
    );
}

const MOD: usize = 2 * 3 * 5 * 7;
const MOD_I64: i64 = 2 * 3 * 5 * 7;

fn solve(s: &[char]) -> u64
{
    /*
    dyn[i][x] := number of ways we get an expression evaluating
      to x (mod 210) if we only consider the first i
      characters of the string. (*)
      */
    let mut dp: Vec<Vec<u64>> = vec![vec![0; MOD]; s.len() + 1];

    dp[0][0] = 1;
    for i in 0..s.len() {
        //: Between each two adjacent digits you may choose put a plus sign, a minus sign, or nothing.
        let start_sgn = if i == 0 { 1 } else { -1 };
        //insert a + or a -
        for sgn in (start_sgn..=1).step_by(2) {
            let mut cur = 0i64;
            for j in i..s.len() {
                let digit = s[j].to_digit(10).unwrap() as i64;
                //build up the rhs
                cur = (cur * 10 + digit) % MOD_I64;
                for x in 0..MOD_I64 {
                    //x is the lhs basically, up to i characters in s (inclusive)
                    //also move -210 to 0 into something positive for easier calculations
                    let new_x = ((x + sgn * cur + MOD_I64) % MOD_I64) as usize;
                    dp[j + 1][new_x] += dp[i][x as usize];
                }
            }
        }
    }

    let mut ret = 0;
    for x in 0..MOD {
        if x % 2 == 0 || x % 3 == 0 || x % 5 == 0 || x % 7 == 0 {
            ret += dp[s.len()][x];
        }
    }
    ret
}
