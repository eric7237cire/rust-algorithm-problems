use codejam::util::codejam::run_cases;

use bit_set::BitSet;
use codejam::algo::graph::scc::strongly_connected_components;
use codejam::algo::graph::DiGraph;
use codejam::util::grid::Grid;
use itertools::Itertools;
use std::cmp::max;
use std::collections::HashMap;
use std::io::Write;
use std::{u16, usize};

/*

*/
pub fn solve_all_cases()
{
    run_cases(
        &[
            "C-small-practice",
            "C-large-practice"
        ],
        "y2016round1A",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let N: u16 = reader.read_int();

                let bff_list = reader.read_num_line();

                if case_no != 16 {
                    //continue;
                }

                println!("Solving case {}", case_no);

                writeln!(buffer, "Case #{}: {}", case_no, solve(&bff_list)).unwrap();
            }
        },
    );
}

/*
fn solve_brute_force(bff_list: &[u16]) -> u16
{
    let indices: Vec<u16> = (0..bff_list.len()).collect();

    let mut current_max = 0;
    loop
        {
            for pos in 0..bff_list.len()
                {
                    let before = if pos == 0 { bff_list.len() -1 } else {pos-1};
                    let after = if pos == bff_list.len() { 0 } else { pos + 1};

                    let bff = bff_list[indices[pos]] - 1;

                    if bff != indices[before] &&
                }
        }


    if let Some(mut permutation) = permutator.next() {
        for element in &permutation {
            println!("{}", element);
        }

        while permutator.next_with_buffer(&mut permutation) {
            println!("Next iter");
            for element in &permutation {
                println!("{}", element);
            }
        }
    }

}*/

fn solve(bff_list: &[u16]) -> usize
{
    let mut graph = DiGraph::new();

    //2 * paper index = is horizonal
    //2 * paper index = !is horizonal == is vertical

    for (idx, bff) in bff_list.iter().enumerate() {
        graph.add_edge(idx + 1, usize::from(*bff));
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

    let mut max_answer = 0;

    let sccs = strongly_connected_components(&graph);
    println!("Sccs: {:?}", sccs);

    let mut not_in_answer = BitSet::new();

    for (idx, scc) in sccs.iter().enumerate() {
        max_answer = max(max_answer, scc.len());

        //A strongly connected component more than 2 must be by itself
        if scc.len() > 2 {
            not_in_answer.extend(scc.iter().cloned());
        }
    }

    let mut longest_path_ending = vec![0; bff_list.len()+1];

    //let mut paths = Vec::new();

    'node_loop: for node in 1..=bff_list.len() {
        if not_in_answer.contains(node) {
            continue;
        }

        if graph.edges_to(node).next().is_some() {
            continue;
        }

        let mut path = Vec::new();

        let mut node = node;
        path.push(node);

        loop {
            node = graph.edges_from(node).next().unwrap();
            if not_in_answer.contains(node) {
                continue 'node_loop;
            }
            //println!("Node {} Path {:?}", node, path);
            if path.len() >= 2 && path[path.len() - 2] == node {
                break;
            }
            path.push(node);
            //break;
        }

        println!("Found path {:?}", path);
        longest_path_ending[ *path.last().unwrap() ] = max(path.len(),
        longest_path_ending[ *path.last().unwrap() ])
        //paths.push(path);
    }

    println!("Found path lengths {:?}", longest_path_ending);

    let mut scc_2_len = 0;
    for scc in sccs.iter() {

        if scc.len() != 2 {
            continue;
        }
        println!("Looking at scc of len 2: {:?}", scc);
        let mut comp_len = max(longest_path_ending[ scc[0] ] +
            longest_path_ending[ scc[1] ], 2) - 2 ;
        comp_len = max(comp_len, longest_path_ending[ scc[0] ]);
        comp_len = max(comp_len, longest_path_ending[ scc[1] ]);
        comp_len = max(comp_len, 2);

        scc_2_len += comp_len;
    }

    //can also have all sccs of len 2 side by side
    max_answer = max(max_answer, scc_2_len);

    max_answer
    //solve_brute_force(bff_list)
}
