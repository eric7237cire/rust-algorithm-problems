use codejam::util::codejam::run_cases;

use bit_set::BitSet;
use itertools::Itertools;
use std::io::Write;
use std::u16;

/*

*/
pub fn solve_all_cases()
{
    run_cases(
        &[
            "B-small-practice",
            //"A-large-practice"
        ],
        "y2016round1A",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let N: u16 = reader.read_int();

                let papers = (0..2 * N - 1)
                    .map(|_| reader.read_num_line())
                    .collect::<Vec<_>>();

                if case_no > 3 {
                    continue;
                }

                println!("Solving case {}", case_no);

                writeln!(
                    buffer,
                    "Case #{}: {}",
                    case_no,
                    solve(&papers).iter().join(" ")
                )
                .unwrap();
            }
        },
    );
}

fn solve(papers: &[Vec<u16>]) -> Vec<u16>
{
    let N = papers.len();
    let all_choices = vec![vec![-1; 2]; N];

    let mut used = BitSet::new();
    //first find the diagonal values, which must be the least value
    for pos in 0..N {
        let least_value = papers
            .iter()
            .enumerate()
            .filter(|(idx, _)| !used.contains(idx))
            .map(|(_, paper)| paper[pos])
            .min();

        let choices = papers
            .iter()
            .enumerate()
            .filter(|(idx, paper)| paper[pos] == least_value)
            .map(|(idx, _)| idx)
            .collect();

        assert!(choices.len() <= 2 && choices.len() >= 1);

        all_choices[pos] = choices;
    }

    let mut ans = Vec::new();
    ans.push(3);
    ans.push(5);

    ans
}
