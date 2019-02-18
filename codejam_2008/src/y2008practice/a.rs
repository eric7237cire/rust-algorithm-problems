//use bit_set::BitSet;
use codejam::util::codejam::run_cases;
use itertools::Itertools;
use std::io::Write;

/*
Change of base
*/
pub fn solve_all_cases()
{
    run_cases(
        &["A-small-practice", "A-large-practice"],
        "y2008practice",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let in_s = reader.read_string_line();

                if case_no != 3 {
                    // continue;
                }

                println!("Solving case {}", case_no);

                writeln!(
                    buffer,
                    "Case #{}: {}",
                    case_no,
                    solve(&in_s[0], &in_s[1], &in_s[2])
                )
                .unwrap();
            }
        },
    );
}

fn solve(alien_number: &str, source_lang: &str, target_lang: &str) -> String
{
    let source_base = source_lang.len();
    let target_base = target_lang.len();

    let mut al_converted = 0;

    //#First convert alienNumber
    for (i, digit) in alien_number.chars().enumerate() {
        debug!("Digit is {}", digit);
        let digit_value = source_lang.chars().position(|ch| ch == digit).unwrap();
        debug!("Digit Value {}", digit_value);
        al_converted +=
            digit_value * source_base.pow((alien_number.len() - i - 1) as u32);
    }

    debug!("Alien # converted is {}", al_converted);

    let mut tl_converted = Vec::new();
    while al_converted > 0 {
        let digit = al_converted % target_base;
        debug!("tlDigit {}", digit);
        tl_converted.push(target_lang.chars().skip(digit).take(1).next().unwrap());
        al_converted /= target_base;
    }

    tl_converted.reverse();

    tl_converted.iter().join("")
}
