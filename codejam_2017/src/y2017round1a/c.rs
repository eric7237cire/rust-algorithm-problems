use codejam::util::codejam::run_cases;
use std::cmp;
use std::io::Write;

/*
very hard ad hoc (code not mine, just translation)
*/
pub fn solve_all_cases()
{
    run_cases(
        &["C-small-practice", "C-large-practice"],
        "y2017round1a",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                //handle input / output
                //Hd, Ad, Hk, Ak, B, and D;
                let input: Vec<i64> = reader.read_num_line();

                if case_no != 1 {
                    //        continue;
                }

                println!("Solving case {}", case_no);


                writeln!(buffer, "Case #{}: {}", case_no, solve(
                         input[0], input[1], input[2], input[3], input[4], input[5],
                    )).unwrap();
            }
        },
    );
}

const R: i64 = 100;

fn solve( h_d: i64, a_d: i64, h_k: i64, a_k: i64, bb: i64, dd: i64) -> String
{
    let max_turn = 10i64.pow(15);
    let mut a_k = a_k;
    let b;
    if bb == 0 {
        b = 0;
    } else {
        let z: f64 = (((bb * h_k) as f64).sqrt() - a_d as f64) / bb as f64;
        if z < 0f64 {
            b = 0;
        } else {
            let b1 = z as i64;
            let b2 = b1 + 1;
            if (h_k + b1 * bb + a_d - 1) / (b1 * bb + a_d) + b1
                <= (h_k + b2 * bb + a_d - 1) / (b2 * bb + a_d) + b2
            {
                b = b1;
            } else {
                b = b2;
            }
        }
    }
    let a = (h_k + b * bb + a_d - 1) / (b * bb + a_d);
    let ab = a + b;

    let mut minturn = max_turn;
    if ab <= (h_d - 1) / a_k + 1 {
        minturn = ab;
    } else if dd == 0 {
        let u = (h_d - a_k - 1) / a_k;
        if u <= 0 && h_k > a_d {
            minturn = max_turn;
        } else {
            let w = ab - (h_d - 1) / a_k;
            minturn = (h_d - 1) / a_k + w + (w + u - 2) / u;
        }
    } else if h_k > a_d && a_k - dd >= h_d {
        minturn = max_turn;
    } else {
        let mut z = 0;
        let mut last_was_heal = false;
        let mut h_i = h_d;
        let mut lastu: i64 = -1;
        while a_k > 0 {
            let u = (h_d - a_k - 1) / a_k;
            if ab <= (h_i - 1) / a_k + 1 {
                minturn = cmp::min(minturn, z + ab);
            } else if u > 0 && !last_was_heal && u as i64 != lastu {
                let w = ab - (h_i - 1) / a_k;
                minturn = cmp::min(minturn, z + (h_i - 1) / a_k + w + (w + u - 2) / u);
            }
            if h_i <= a_k - dd {
                if last_was_heal {
                    break;
                }
                z += 1;
                last_was_heal = true;
                h_i = h_d - a_k;
                if a_k >= R * dd && u == (h_d - (a_k - R * dd) - 1) / (a_k - R * dd) {
                    let tt = R / u;
                    z += tt * (u + 1);
                    a_k -= tt * u * dd;
                    h_i = h_d - a_k;
                }
            } else {
                last_was_heal = false;
                z += 1;
                a_k -= dd;
                h_i -= a_k;
            }
            lastu = u as i64;
        }
        if a_k <= 0 {
            minturn = cmp::min(minturn, z + ab);
        }
    }

    if minturn == max_turn {
        format!("{}", "IMPOSSIBLE")
    } else {
        format!("{}", minturn)
    }
}
