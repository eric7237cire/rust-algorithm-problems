use codejam::util::codejam::run_cases;

use bit_set::BitSet;
use codejam::algo::graph::scc::strongly_connected_components;
use codejam::algo::graph::DiGraph;
use codejam::util::grid::Grid;
use itertools::Itertools;
use std::collections::HashMap;
use std::io::Write;
use std::{u16, usize};

/*
I used 2SAT and backtracking though there was also a much easier solution to just count odd counts of the
values.  Oh well...
*/
pub fn solve_all_cases()
{
    run_cases(
        &["B-small-practice", "B-large-practice"],
        "y2016round1A",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let N: u16 = reader.read_int();

                let papers = (0..2 * N - 1)
                    .map(|_| reader.read_num_line())
                    .collect::<Vec<_>>();

                if case_no != 4 {
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

#[allow(dead_code)]
fn backtracking(
    horizonal_choices: &mut Vec<usize>,
    all_choices: &[Vec<usize>],
    papers: &[Vec<u16>],
) -> bool
{
    //Reject invalid solutions
    for column in 0..horizonal_choices.len() {
        if all_choices[column].len() == 1 {
            continue;
        }

        let other_choice = if horizonal_choices[column] == all_choices[column][0] {
            all_choices[column][1]
        } else {
            all_choices[column][0]
        };

        //only need to verify the new column and new row; everything else
        //has already been checked
        let lower_bound = if column == horizonal_choices.len() - 1 {
            0
        } else {
            horizonal_choices.len() - 1
        };

        for row in lower_bound..horizonal_choices.len() {
            if papers[horizonal_choices[row]][column] != papers[other_choice][row] {
                return false;
            }
        }
    }

    if horizonal_choices.len() == all_choices.len() {
        return true;
    }

    let current_pos = horizonal_choices.len();

    for choice in all_choices[current_pos].iter() {
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
    for (i, p) in papers.iter().enumerate() {
        println!("Paper {}: {:?}", i, p);
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
            .min()
            .unwrap();

        //println!("Value for diag pos {} = {}", pos, least_value);

        let choices: Vec<_> = papers
            .iter()
            .enumerate()
            .filter(|(_idx, paper)| paper[pos] == least_value)
            .map(|(idx, _)| idx)
            .collect();

        assert!(choices.len() <= 2 && !choices.is_empty());

        for choice in choices.iter() {
            used.insert(*choice);
        }

        all_choices[pos] = choices;
    }

    println!("All choices: {:?}", all_choices);

    let mut graph = DiGraph::new();

    //2 * paper index = is horizonal
    //2 * paper index = !is horizonal == is vertical

    for (pos, choices) in all_choices.iter().enumerate() {
        if choices.len() == 1 {
            let choice = 2 * choices[0];
            graph.add_edge(choice ^ 1, choice);

            add_clauses(choices[0], pos, &all_choices, papers, &mut graph);
        } else {
            assert_eq!(2, choices.len());
            let choice1 = 2 * choices[0];
            let choice2 = 2 * choices[1];
            graph.add_edge(choice1 ^ 1, choice2);
            graph.add_edge(choice2 ^ 1, choice1);
            graph.add_edge(choice1, choice2 ^ 1);
            graph.add_edge(choice2, choice1 ^ 1);

            add_clauses(choices[0], pos, &all_choices, papers, &mut graph);
            add_clauses(choices[1], pos, &all_choices, papers, &mut graph);
        }
    }
    /*
    for (from, to) in graph.edges()
    {
        let from_idx = from / 2;
        let from_is_h = if from % 2 == 0 { "Horizontal" } else {"Vertical"};
        let to_idx = to / 2;
        let to_is_h = if to % 2 == 0  { "Horizontal" } else {"Vertical"};
        println!("From ({}) {} {} implies ({}) {} {}",
        from,
        from_idx, from_is_h, to, to_idx, to_is_h);
    }*/

    let sccs = strongly_connected_components(&graph);
    println!("Sccs: {:?}", sccs);

    let mut node_to_scc_component = HashMap::new();
    for (idx, scc) in sccs.iter().enumerate() {
        for node in scc.iter() {
            node_to_scc_component.insert(node, idx);
        }
    }

    let mut horizonal_choices = Vec::new();
    //backtracking(&mut horizonal_choices, &all_choices, papers);

    for (pos, choices) in all_choices.iter().enumerate() {
        let paper_idx = choices[0];
        let scc_true = node_to_scc_component[&(2 * paper_idx)];
        let scc_false = node_to_scc_component[&(2 * paper_idx + 1)];

        if scc_true == scc_false {
            panic!("Not satisfiable");
        } else {
            println!(
                "In choice {:?} for diag pos {},  Paper {} is {}",
                choices,
                pos,
                paper_idx,
                if scc_true < scc_false {
                    "Horizonal"
                } else {
                    "Vertical"
                }
            );
        }

        assert!(scc_true != scc_false);
        if scc_true < scc_false {
            horizonal_choices.push(paper_idx);
        } else {
            horizonal_choices.push(choices[1]);
        }
    }

    let mut g: Grid<u16> = Grid::new(N, N);

    for (h_idx, h) in horizonal_choices.iter().enumerate() {
        for (c_idx, c) in papers[*h].iter().enumerate() {
            g[(h_idx, c_idx)] = *c;
        }
    }

    println!("Grid\n{:#.5?}", g);

    //println!("Horizonal choices: {:?}", horizonal_choices);

    //Which vertical column is missing?
    let column_index = all_choices
        .iter()
        .position(|choices| choices.len() == 1)
        .unwrap();

    horizonal_choices
        .iter()
        .map(|choice| papers[*choice][column_index])
        .collect()
}

fn add_clauses(
    horizonal_choice: usize,
    row: usize,
    all_choices: &[Vec<usize>],
    papers: &[Vec<u16>],
    graph: &mut DiGraph,
)
{
    for (column, choices) in all_choices.iter().enumerate() {
        for choice in choices.iter() {
            if papers[*choice][row] != papers[horizonal_choice][column] {
                //must be both horizonal or both vertical
                graph.add_edge(2 * horizonal_choice, 2 * choice);
                graph.add_edge(2 * choice, 2 * horizonal_choice);
                graph.add_edge(2 * horizonal_choice + 1, 2 * choice + 1);
                graph.add_edge(2 * choice + 1, 2 * horizonal_choice + 1);
            }
        }
    }
}
