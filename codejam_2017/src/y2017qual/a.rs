use codejam::util::codejam::run_cases;
use std::io::Write;

//use bit_vec::BitVec;

//Greedy algorithm

pub fn solve_all_cases()
{
    run_cases(
        &["A-small-practice", "A-large-practice"],
        "y2017qual",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let s: Vec<_> = reader.read_string_line();
                let k = s[1].trim().parse::<usize>().unwrap();
                let s = s[0].trim();
                let mut v: Vec<bool> = s.chars().map(|x| x == '+').collect();

                if case_no != 1 {
                    //        continue;
                }

                println!("Solving case {}", case_no);

                writeln!(
                    buffer,
                    "Case #{}: {}",
                    case_no,
                    match solve(&mut v, k) {
                        None => "IMPOSSIBLE".to_string(),
                        Some(ans) => ans.to_string(),
                    }
                )
                .unwrap();
            }
        },
    );
}

fn solve(pancake_row: &mut [bool], k: usize) -> Option<usize>
{
    let mut moves = 0;
    //proceed left to right, flipping as we must
    for i in 0..pancake_row.len() - k + 1 {
        if !pancake_row[i] {
            moves += 1;
            for j in i..i + k {
                pancake_row[j] = !pancake_row[j];
            }
        }
    }

    //if everything is how it should be, we succeeded
    match pancake_row.iter().all(|&x| x) {
        true => Some(moves),
        false => None,
    }
}
