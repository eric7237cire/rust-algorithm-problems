use codejam::util::codejam::run_cases;

use itertools::Itertools;
use std::io::Write;

/*

*/
pub fn solve_all_cases()
{
    run_cases(
        &["A-small-practice",
            //"A-large-practice"
             ],
        "y2016round1C",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let N : usize= reader.read_int();

                let P = reader.read_num_line();
                assert_eq!(N, P.len());

                if case_no != 3 {
                    // continue;
                }

                //println!("Solving case {}", case_no);

                writeln!(buffer, "Case #{}: {}", case_no, solve(&P)).unwrap();
            }
        },
    );
}

fn solve(P: &[u16]) -> String
{
    let mut ans : Vec<String> = Vec::new();

    let mut P: Vec< ( u16, char) > = P.iter().cloned().zip( (0..26).map(|ascii| char::from(b'A' + ascii))).collect();

    let mut total: u16 = P.iter().fold(0, |acc, (count, _)| acc+count);

    while total > 0 {

        let upper_limit = 1 + total / 2;

        println!("Count of {}.  total={} upper limit = {}",
P.iter().map(| (ch, count)| format!("{} = {}", ch, count)).join("; "), total, upper_limit);


        if P.iter().any( |(count, _)| *count >= upper_limit) {
            panic!("Invalid configuration");
        }

        P.sort();
        P.reverse();

        if P[0].0 == P[1].0 && (P.len() <= 2 || P[2].0 != P[1].0) {
            ans.push(format!("{}{}", P[0].1, P[1].1));
            P[0].0 -= 1;
            P[1].0 -= 1;
        } else {
            ans.push(P[0].1.to_string());
            P[0].0 -= 1;
        }

        total = P.iter().fold(0, |acc, (count, _)| acc+count);
    }

    ans.iter().join(" ")
}
