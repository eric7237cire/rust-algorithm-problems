use codejam::util::codejam::run_cases;
//use std::cmp::max;
//use itertools::Itertools;
use std::io::Write;
//use std::mem::swap;

//use std::cmp::max;
//use std::cmp::Ordering;
//use std::collections::BinaryHeap;
//use std::collections::HashMap;
use std::usize;

use codejam::algo::graph::disjointset::DisjointSet;
use codejam::util::bitvec64::BitVec64;

//use permutohedron::LexicalPermutation;

/*

*/
pub fn solve_all_cases()
{
    run_cases(
        &[
            "D-small-practice",
            //"C-large-practice"
        ],
        "y2016round2",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let N = reader.read_int();

                let workers: Vec<BitVec64> =
                    (0..N).map(|_| reader.read_chars(N).into_iter().collect::<BitVec64>()).collect();

                if case_no > 5 {
                     continue;
                }

                println!("Solving case {}", case_no);

                writeln!(
                    buffer,
                    "Case #{}:\n{}",
                    case_no,
                    solve(&workers) //solve_brute_force(R, C, &lovers)
                )
                .unwrap();
            }
        },
    );
}

struct Node
{
    num_machines: i8,
    num_workers: i8,
}

fn solve(workers: &[BitVec64]) -> usize
{
    let N = workers.len();

    let mut ds = DisjointSet::new(N);

    for worker in workers.iter() {
        let mut first_machine = N;
        for machine in 0..N {
            if worker.getb(machine) {
                if first_machine == N {
                    first_machine = machine;
                }
                ds.merge_sets(first_machine, machine);
            }
        }
    }

    for machine in 0..N {
        debug!("Machine {} set # {}", machine, ds.get_repr(machine));
    }
    for machine in 0..N {
        debug!("Machine {} set # {}", machine, ds.get_repr(machine));
    }


    3
}
