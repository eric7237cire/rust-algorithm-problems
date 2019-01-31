use codejam::util::codejam::run_cases;

use bit_set::BitSet;
use itertools::Itertools;
use std::io::Write;
use std::{u16,usize};

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
                    //continue;
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

fn backtracking( horizonal_choices: &mut Vec<usize>, all_choices: &Vec<Vec<usize>>,
    papers: &[Vec<u16>]) -> bool 
{
    let N = all_choices.len();

    //Reject invalid solutions
    for column in 0..horizonal_choices.len()
    {
        if all_choices[column].len() == 1 {
            continue;
        }

        let other_choice = if horizonal_choices[column] == all_choices[column][0] {
            all_choices[column][1]
        } else { all_choices[column][0] };

        for row in 0..horizonal_choices.len()
        {
            if papers[horizonal_choices[row]][column] != 
            papers[other_choice][row] {
                return false;
            }
        }
    }

    if horizonal_choices.len() == all_choices.len() {
        return true;
    }

    let current_pos = horizonal_choices.len();

    for choice in all_choices[current_pos].iter()
    {
        horizonal_choices.push(*choice);
        if backtracking(horizonal_choices, all_choices, papers) {
            return true;
        }
        horizonal_choices.pop();
    }

    false
}


fn solve(papers: &[Vec<u16>]) -> Vec<u16>
{
    let N = papers[0].len();
    for (i, p) in papers.iter().enumerate()
    {
        //println!("Paper {}: {:?}", i, p);
    }
    let mut all_choices = vec![vec![usize::MAX; 2]; N];

    let mut used = BitSet::new();
    //first find the diagonal values, which must be the least value
    for pos in 0..N {
        let least_value = papers
            .iter()
            .enumerate()
            .filter(|(idx, _)| !used.contains(*idx))
            .map(|(_, paper)| paper[pos])
            .min().unwrap();

        //println!("Value for diag pos {} = {}", pos, least_value);

        let choices: Vec<_> = papers
            .iter()
            .enumerate()
            .filter(|(idx, paper)| paper[pos] == least_value)
            .map(|(idx, _)| idx)
            .collect();

        assert!(choices.len() <= 2 && !choices.is_empty());

        for choice in choices.iter() {
            used.insert(*choice);
        }

        all_choices[pos] = choices;
    }

    //println!("All choices: {:?}", all_choices);

    let mut horizonal_choices = Vec::new();
    backtracking(&mut horizonal_choices, &all_choices, papers);

    //println!("Horizonal choices: {:?}", horizonal_choices);

    //Which vertical column is missing?
    let column_index = all_choices.iter().position( |choices| choices.len() == 1).unwrap();

    horizonal_choices.iter().map( |choice| papers[*choice][column_index] ).collect()
    
}
