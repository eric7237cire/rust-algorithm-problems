use codejam::util::codejam::run_cases;

use bit_set::BitSet;
use codejam::algo::graph::scc::strongly_connected_components;
use codejam::algo::graph::DiGraph;
use std::cmp::max;
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
                let _N: u16 = reader.read_int();

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


fn solve(bff_list: &[u16]) -> usize
{
    let mut graph = DiGraph::new();

    for (idx, bff) in bff_list.iter().enumerate() {
        graph.add_edge(idx + 1, usize::from(*bff));
    }

    let sccs = strongly_connected_components(&graph);
    //println!("Sccs: {:?}", sccs);

    let mut in_scc_larger_than_2 = BitSet::new();

    //the largest scc can be used for the entire circle
    let largest_scc_len = sccs.iter().map( |scc| {
                
        //A strongly connected component more than 2 must be by itself
        if scc.len() > 2 {
            in_scc_larger_than_2.extend(scc.iter().cloned());
        }

        scc.len()
    }).max().unwrap();

    let mut longest_path_ending = vec![0; bff_list.len()+1];

    'node_loop: for node in 1..=bff_list.len() {
        if in_scc_larger_than_2.contains(node) {
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
            if in_scc_larger_than_2.contains(node) {
                continue 'node_loop;
            }
            //println!("Node {} Path {:?}", node, path);
            if path.len() >= 2 && path[path.len() - 2] == node {
                break;
            }
            path.push(node);
        }

        //println!("Found path {:?}", path);
        longest_path_ending[ *path.last().unwrap() ] = max(path.len(),
        longest_path_ending[ *path.last().unwrap() ]);        
    }

    //println!("Found path lengths {:?}", longest_path_ending);

    let scc_2_len = sccs.iter().map(|scc| {

        if scc.len() != 2 {
            return 0;
        }
        //println!("Looking at scc of len 2: {:?}", scc);
        let mut comp_len = max(longest_path_ending[ scc[0] ] +
            longest_path_ending[ scc[1] ], 2) - 2 ;
        comp_len = max(comp_len, longest_path_ending[ scc[0] ]);
        comp_len = max(comp_len, longest_path_ending[ scc[1] ]);
        comp_len = max(comp_len, 2);

        comp_len
    }).sum();

    max(largest_scc_len, scc_2_len)
}
