use codejam::util::codejam::run_cases;

use itertools::Itertools;
use std::cmp::min;
use std::i64;
use std::io::Write;

/*

*/
pub fn solve_all_cases()
{
    run_cases(
        &[
            "B-small-practice",
            "B-large-practice"
        ],
        "y2016round1B",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let scores = reader.read_string_line();

                assert_eq!(scores.len(), 2, "{}", scores[0]);

                if case_no != 107 {
                    //continue;
                }

                println!("Solving case {}", case_no);

                writeln!(
                    buffer,
                    "Case #{}: {}",
                    case_no,
                    solve(&scores[0], &scores[1])
                )
                .unwrap();
            }
        },
    );
}

fn get_digits(num: i64, len_num: usize) -> Vec<i8>
{
    let mut digits = Vec::new();
    let mut num = num;
    while num > 0 {
        digits.push((num % 10) as i8);
        num /= 10
    }

    while digits.len() < len_num {
        digits.push(0)
    }

    digits.reverse();

    digits
}

fn str_to_digits(digit_string: &str) -> Vec<i8>
{
    digit_string
        .chars()
        .map(|ch| {
            if ch.is_ascii_digit() {
                ch.to_digit(10).unwrap() as i8
            } else {
                -1
            }
        })
        .collect()
}

#[derive(Default, Debug)]
struct DigitInfo
{
    max_c: i8,
    min_c: i8,
    max_j: i8,
    min_j: i8,
    fixed_c: Option<i8>,
    fixed_j: Option<i8>,

    mul_base: i64,
    max_diff: i64,
    min_diff: i64,
}

impl DigitInfo
{
    fn max_j(&self) -> i8
    {
        if let Some(j) = self.fixed_j {
            j
        } else {
            9
        }
    }
    fn min_j(&self) -> i8
    {
        if let Some(j) = self.fixed_j {
            j
        } else {
            0
        }
    }
    fn max_c(&self) -> i8
    {
        if let Some(c) = self.fixed_c {
            c
        } else {
            9
        }
    }
    fn min_c(&self) -> i8
    {
        if let Some(c) = self.fixed_c {
            c
        } else {
            0
        }
    }
}

#[deny(clippy::collpasable_if, clippy::cyclomatic_complexity)]
fn solve(C: &str, J: &str) -> String
{
    assert_eq!(C.len(), J.len());

    let powers_10: Vec<i64> = (0..C.len()).rev().map(|i| 10i64.pow(i as u32)).collect();

    let info: Vec<DigitInfo> = izip!(C.chars(), J.chars(), powers_10.iter())
        .map(|(ch_c, ch_j, &pow10)| {
            let mut digit_info: DigitInfo = Default::default();
            if ch_c == '?' {
                digit_info.max_c = 9;
                digit_info.min_c = 0;

                digit_info.fixed_c = None;
            } else {
                let digit = ch_c.to_digit(10).unwrap() as i8;
                digit_info.max_c = digit;
                digit_info.min_c = digit;
                digit_info.fixed_c = Some(digit);
            }

            if ch_j == '?' {
                digit_info.max_j = 9;
                digit_info.min_j = 0;
                digit_info.fixed_j = None;
            } else {
                let digit = ch_j.to_digit(10).unwrap() as i8;
                digit_info.max_j = digit;
                digit_info.min_j = digit;
                digit_info.fixed_j = Some(digit);
            }

            digit_info.mul_base = pow10;
            digit_info.max_diff =
                i64::from(digit_info.max_c) * pow10 - i64::from(digit_info.min_j) * pow10;
            digit_info.min_diff =
                i64::from(digit_info.min_c) * pow10 - i64::from(digit_info.max_j) * pow10;

            digit_info
        })
        .collect();

    // info.push(Default::default());

    for di in info.iter() {
        println!("Digit Info: {:?}", di);
    }

    let mut cumulative_min_max = Vec::new();
    cumulative_min_max.push([0, 0, 0]);

    for (di, &pow10) in info.iter().rev().zip(powers_10.iter().rev()) {
        let last = cumulative_min_max.last().unwrap();

        let last_upper_bound = last[0];
        let last_sm_mag_diff = last[1];
        let last_lower_bound = last[2];

        let smallest_mag_diff = if let (Some(c), Some(j)) = (di.fixed_c, di.fixed_j) {
            let s = pow10 * i64::from(c - j);
            if s < 0 {
                s + last[0]
            } else if s > 0 {
                //reduce the pos swing by current min
                s + last[2]
            } else {
                s + last_sm_mag_diff
            }

        } else if last_sm_mag_diff < 0 && pow10 + last_lower_bound < last_sm_mag_diff.abs() && di.max_c() > di.min_j() {
            //basically making c greater
            assert!(pow10 + last_lower_bound > 0);
            pow10 + last_lower_bound
        } else if last_sm_mag_diff > 0 && pow10 - last_upper_bound < last_sm_mag_diff && di.max_j() > di.min_c() {
            //making j greater
            assert!(last_upper_bound - pow10 < 0);
            last_upper_bound - pow10
        } else {
            last_sm_mag_diff
        };

        cumulative_min_max.push([
            last[0] + di.max_diff,
            smallest_mag_diff,
            last[2] + di.min_diff,
        ]);
    }

    cumulative_min_max.reverse();

    let mut c_digits: Vec<i8> = Vec::new();
    let mut j_digits: Vec<i8> = Vec::new();

    for cmm in cumulative_min_max.iter() {
        println!(
            " Cumulative max {} abs min {} min {}",
            cmm[0], cmm[1], cmm[2]
        );
    }

    assert_eq!(cumulative_min_max.len(), C.len() + 1);

    println!("Determing {} and {}", C, J);

    let mut current_diff = 0;

    for pos in 0..C.len() {
        let di = &info[pos];

        println!(
            "C={}... J={}... Digit {}",
            c_digits.iter().join(""),
            j_digits.iter().join(""),
            pos
        );

        if current_diff > 0 {
            //C is currently greater
            c_digits.push(di.min_c());
            j_digits.push(di.max_j());
            continue;
        }

        if current_diff < 0 {
            //C is currently lesser
            c_digits.push(di.max_c());
            j_digits.push(di.min_j());
            continue;
        }

        if di.min_c == di.max_c && di.min_j == di.max_j {
            //no choice
            c_digits.push(di.max_c);
            j_digits.push(di.max_j);
            current_diff = di.max_c as i8 - di.max_j as i8;
            continue;
        }

        let half = di.mul_base / 2;

        let diff_upper_bound = cumulative_min_max[pos + 1][0];
        let min_diff = cumulative_min_max[pos + 1][1];
        let prev_min_diff = cumulative_min_max[pos][1];
        let diff_lower_bound = cumulative_min_max[pos + 1][2];

        println!(
            "diff range is {} to {}.  Signed min mag diff {}, prev {}",
            diff_lower_bound, diff_upper_bound, min_diff, prev_min_diff
        );

        if let Some(c) = di.fixed_c {
            // c is fixed
            c_digits.push(c);

            //1 higher, to be avoided since it makes c greater
            //only do it if its what lowers the diff
            if pos < C.len() - 1
                && c < 9
                //&& di.mul_base - diff_upper_bound == min_diff
                //going from neg to pos
                && prev_min_diff < min_diff 
            {
                j_digits.push(c + 1);
                current_diff = -1;
            }
            //1 lower, if we can do it, since it makes c lower
            else if pos < C.len() - 1 && c > 0 && (prev_min_diff > min_diff || min_diff == -half || diff_lower_bound + di.mul_base == min_diff )
            {
                j_digits.push(c - 1);
                current_diff = 1;
            } else {
                j_digits.push(c);
                assert_eq!(0, current_diff);
            }
        } else if let Some(j) = di.fixed_j {
            j_digits.push(j);

            //c 1 lower, if we can do it
            if pos < C.len() - 1
                && j > 0
                && (prev_min_diff < min_diff
                    || min_diff == half
                    || diff_upper_bound - di.mul_base == min_diff)
            {
                c_digits.push(j - 1);
                current_diff = -1;
            }
            //c 1 higher, to be avoided
            else if pos < C.len() - 1 && j < 9 && prev_min_diff > min_diff {
                c_digits.push(j + 1);
                current_diff = 1;
            } else {
                c_digits.push(j);
                assert_eq!(0, current_diff);
            }
        } else {
            //both flexible

            //if j can be 1 higher, do it since this will minimized c
            if pos < C.len() - 1 && prev_min_diff < min_diff {
                c_digits.push(0);
                j_digits.push(1);
                current_diff = -1;
            }
            // if c must be 1 higher, to be avoided
            else if pos < C.len() - 1 && prev_min_diff > min_diff {
                c_digits.push(1);
                j_digits.push(0);
                current_diff = 1;
            } else {
                c_digits.push(0);
                j_digits.push(0);
                assert_eq!(0, current_diff);
            }
        }
    }

    println!(
        "Ans C={}... J={}",
        c_digits.iter().join(""),
        j_digits.iter().join("")
    );

    format!("{} {}", c_digits.iter().join(""), j_digits.iter().join(""))
}

fn solve_brute_force(C: &str, J: &str) -> String
{
    let c_digit_mask = str_to_digits(C);
    let j_digit_mask = str_to_digits(J);

    assert_eq!(C.len(), J.len());

    let upper_limit: i64 = 10i64.pow(C.len() as u32) as i64;

    let mut best_solution = (i64::MAX, i64::MAX, i64::MAX);
    let mut best_c_digits = c_digit_mask.clone();
    let mut best_j_digits = j_digit_mask.clone();

    for c in 0..upper_limit {
        let c_digits = get_digits(c, C.len());

        if c_digits
            .iter()
            .zip(c_digit_mask.iter())
            .any(|(&dig, &mask)| mask != -1 && mask != dig)
        {
            continue;
        }

        for j in 0..upper_limit {
            let j_digits = get_digits(j, J.len());

            if j_digits
                .iter()
                .zip(j_digit_mask.iter())
                .any(|(&dig, &mask)| mask != -1 && mask != dig)
            {
                continue;
            }

            let sol = ((c - j).abs(), c, j);

            if sol < best_solution {
                best_solution = sol;
                best_c_digits = c_digits.clone();
                best_j_digits = j_digits;
            }
        }
    }

    format!(
        "{} {}",
        best_c_digits.iter().join(""),
        best_j_digits.iter().join("")
    )
}
