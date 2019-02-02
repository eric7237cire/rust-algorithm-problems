use codejam::util::codejam::run_cases;

use itertools::Itertools;
use num_traits::real::Real;
use std::collections::HashMap;
use std::io::Write;
use std::i64;

/*

*/
pub fn solve_all_cases()
{
    run_cases(
        &[
            "B-small-practice",
            //"A-large-practice"
        ],
        "y2016round1B",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let scores = reader.read_string_line();

                assert_eq!(scores.len(), 2, "{}", scores[0]);

                if case_no != 3 {
                    // continue;
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
        digits.push((num % 10) as i8 );
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

fn solve(C: &str, J: &str) -> String
{
    let c_digit_mask = str_to_digits(C);
    let j_digit_mask = str_to_digits(J);

    assert_eq!(C.len(), J.len());

    let upper_limit:i64 = 10i64.pow(C.len() as u32) as i64;

    let mut best_solution = (i64::MAX, i64::MAX, i64::MAX);
    let mut best_c_digits= c_digit_mask.clone();
    let mut best_j_digits= j_digit_mask.clone();

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
