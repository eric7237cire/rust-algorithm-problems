use codejam::util::codejam::run_cases;
use std::io::Write;

/*
Conjugation
Binomial expansion
Cycles
Math
*/
pub fn solve_all_cases()
{
    run_cases(
        &["C-small-practice", "C-large-practice"],
        "y2008round1a",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let n: u64 = reader.read_int();

                if case_no != 3 {
                    // continue;
                }

                println!("Solving case {}", case_no);

                writeln!(buffer, "Case #{}: {:0>3}", case_no, solve(n)).unwrap();
            }
        },
    );
}

fn solve(n: u64) -> u64
{
    let mut n = n;

    //initialize to (3 + sqrt(5) ) ^ 0, represented as integer_part * radical_coeff * sqrt(5)
    let mut integer_part = 1;
    let mut radical_coeff = 0;

    //17 same as 117, so pattern repeats

    if (n > 117) {
        n = 17 + (n - 17) % 100;
    }

    for _ in 0..n {
        let new_integer_part = 3 * integer_part + 5 * radical_coeff;
        let new_radical_coeff = integer_part + 3 * radical_coeff;

        integer_part = new_integer_part % 1000;
        radical_coeff = new_radical_coeff % 1000;
    }

    //printf("2 * X^n - 1 = %lld\n", 2*integer_part-1);
    // Xn is actually the first integer greater than αn. Thus we may just focus on computing the last three digits of X.
    (2 * integer_part - 1) % 1000
}
