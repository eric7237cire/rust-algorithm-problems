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
        "y2008round1a",
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

fn solve( Hd: i64, Ad: i64, Hk: i64, Ak: i64, B: i64, D: i64) -> String
{
    let MAX_TURN = 10i64.pow(15);
    let mut Ak = Ak;
    let b;
    if B == 0 {
        b = 0;
    } else {
        let z: f64 = (((B * Hk) as f64).sqrt() - Ad as f64) / B as f64;
        if z < 0f64 {
            b = 0;
        } else {
            let b1 = z as i64;
            let b2 = b1 + 1;
            if (Hk + b1 * B + Ad - 1) / (b1 * B + Ad) + b1
                <= (Hk + b2 * B + Ad - 1) / (b2 * B + Ad) + b2
            {
                b = b1;
            } else {
                b = b2;
            }
        }
    }
    let a = (Hk + b * B + Ad - 1) / (b * B + Ad);
    let ab = a + b;

    let mut minturn = MAX_TURN;
    if ab <= (Hd - 1) / Ak + 1 {
        minturn = ab;
    } else if D == 0 {
        let u = (Hd - Ak - 1) / Ak;
        if u <= 0 && Hk > Ad {
            minturn = MAX_TURN;
        } else {
            let w = ab - (Hd - 1) / Ak;
            minturn = (Hd - 1) / Ak + w + (w + u - 2) / u;
        }
    } else if Hk > Ad && Ak - D >= Hd {
        minturn = MAX_TURN;
    } else {
        let mut z = 0;
        let mut last_was_heal = false;
        let mut Hi = Hd;
        let mut lastu: i64 = -1;
        while Ak > 0 {
            let u = (Hd - Ak - 1) / Ak;
            if ab <= (Hi - 1) / Ak + 1 {
                minturn = cmp::min(minturn, z + ab);
            } else if u > 0 && !last_was_heal && u as i64 != lastu {
                let w = ab - (Hi - 1) / Ak;
                minturn = cmp::min(minturn, z + (Hi - 1) / Ak + w + (w + u - 2) / u);
            }
            if Hi <= Ak - D {
                if last_was_heal {
                    break;
                }
                z += 1;
                last_was_heal = true;
                Hi = Hd - Ak;
                if Ak >= R * D && u == (Hd - (Ak - R * D) - 1) / (Ak - R * D) {
                    let tt = R / u;
                    z += tt * (u + 1);
                    Ak -= tt * u * D;
                    Hi = Hd - Ak;
                }
            } else {
                last_was_heal = false;
                z += 1;
                Ak -= D;
                Hi -= Ak;
            }
            lastu = u as i64;
        }
        if Ak <= 0 {
            minturn = cmp::min(minturn, z + ab);
        }
    }

    if minturn == MAX_TURN {
        format!("{}", "IMPOSSIBLE")
    } else {
        format!("{}", minturn)
    }
}
