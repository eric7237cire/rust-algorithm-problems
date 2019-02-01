use codejam::util::codejam::run_cases;

use itertools::Itertools;
use std::collections::HashMap;
use std::io::Write;

/*

*/
pub fn solve_all_cases()
{
    run_cases(
        &["A-small-practice", "A-large-practice"],
        "y2016round1B",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let S = reader.read_string();

                if case_no != 3 {
                    // continue;
                }

                println!("Solving case {}", case_no);

                writeln!(buffer, "Case #{}: {}", case_no, solve(&S)).unwrap();
            }
        },
    );
}

fn solve(S: &str) -> String
{
    let mut char_counts: HashMap<char, usize> = HashMap::new();

    for ch in S.chars() {
        *char_counts.entry(ch).or_insert(0) += 1;
    }

    println!("{:?}", char_counts);

    let mut digits = Vec::new();

    if let Some(&z_count) = char_counts.get(&'Z') {
        for ch in "ZERO".chars() {
            *char_counts.entry(ch).or_default() -= z_count;
        }
        for _ in 0..z_count {
            digits.push(0);
        }
    }

    digits.sort();

    digits.iter().join("")
}
