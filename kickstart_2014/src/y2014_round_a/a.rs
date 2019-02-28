//use bit_set::BitSet;
use codejam::util::bitvec64::BitVec64;
use codejam::util::codejam::run_cases;
use itertools::Itertools;
use std::io::Write;
use std::usize;

/*

*/
pub fn solve_all_cases()
{
    run_cases(
        &[
            "A-sample",     "A-small-practice", "A-large-practice"
        ],
        "y2014_round_a",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let in_line = reader.read_string_line();
                let n: usize = in_line[0].parse().unwrap();

                let seq: Vec<usize> = in_line
                    .iter()
                    .skip(1)
                    .map(|s| s.chars().collect::<BitVec64>().data)
                    .collect();

                assert_eq!(n, seq.len());

                if case_no != 3 {
                    // continue;
                }

                println!("Solving case {}", case_no);

                writeln!(buffer, "Case #{}: {}", case_no, solve(&seq)).unwrap();
            }
        },
    );
}

const SEQ_0_9: [usize; 10] = [
    0b_0111111, 0b_0000110, 0b_1011011, 0b_1001111, //3
    0b_1100110, 0b_1101101, 0b_1111101, 0b_0000111, 0b_1111111, 0b_1101111, // 9
];

fn solve(seq: &[usize]) -> String
{
    let mut found = false;
    let mut found_broken = 0;
    let mut next_idx = 0;

    for start in 0..SEQ_0_9.len() {
        for broken_seg in 0..1 << 7 {
            let mut digit_ok = true;

            for ((idx, &check_digit), &test_seq_digit) in SEQ_0_9
                .iter()
                .enumerate()
                .rev()
                .cycle()
                .skip(start)
                .take(seq.len())
                .zip(seq.iter())
            {
                debug!(
                    "Looking at check digit {:0>7b} test digit {:0>7b} idx {}",
                    check_digit, test_seq_digit, idx
                );
                if test_seq_digit != (check_digit & !broken_seg) {
                    digit_ok = false;
                    break;
                }

            }

            if digit_ok && found {
                //must be unique
                found = false;
                break;
            }
            if digit_ok && !found {
                found = true;
                found_broken = broken_seg;
                next_idx = (100 * SEQ_0_9.len() - start - seq.len() - 1) % SEQ_0_9.len();
                debug!(
                    "Found match.  start={} next={} seq len {}",
                    start,
                    next_idx,
                    seq.len()
                );
            }
        }
    }

    if !found {
        "ERROR!".to_string()
    } else {
        let ans = SEQ_0_9[next_idx] & !found_broken;
        (0..7)
            .map(|i| if ans >> i & 1 > 0 { "1" } else { "0" })
            .join("")
    }
}
