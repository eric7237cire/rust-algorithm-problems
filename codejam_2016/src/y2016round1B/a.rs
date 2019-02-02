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

                //println!("Solving case {}", case_no);

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

    //println!("{:?}", char_counts);

    let mut digits = Vec::new();

    let unique_letters = [
        ('Z', "ZERO", 0),
        ('W', "TWO", 2),
        ('G', "EIGHT", 8),
        ('U', "FOUR", 4),
        ('X', "SIX", 6),
        ('H', "THREE", 3),
        //now unique after remove precedent
        ('F', "FIVE", 5),
        ('V', "SEVEN", 7),
        ('O', "ONE", 1),
        ('I', "NINE", 9),
    ];

    //

    for (ch, word, digit) in unique_letters.iter() {
        if let Some(&ch_count) = char_counts.get(ch) {
            for ch in word.chars() {
                *char_counts.entry(ch).or_default() -= ch_count;
            }
            for _ in 0..ch_count {
                digits.push(*digit);
            }
        }
    }

    char_counts.retain(|_, &mut count| count > 0);

    if char_counts.len() > 0 {
        println!("Remaining {:?}", char_counts);
    }

    digits.sort();

    digits.iter().join("")
}
