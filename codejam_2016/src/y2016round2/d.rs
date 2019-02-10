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
use std::cell::RefCell;
use std::rc::Rc;

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

                let workers: Vec<BitVec64> = (0..N)
                    .map(|_| reader.read_chars(N).into_iter().collect::<BitVec64>())
                    .collect();

                if case_no != 6 {
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

#[derive(Default)]
struct Node
{
    machines: BitVec64,
    workers: BitVec64,
}

impl Drop for Node
{
    fn drop(&mut self)
    {
        debug!(
            "Node dropped machines {:b} workers {:b}",
            self.machines, self.workers
        );
    }
}

impl Node
{
    fn num_machines(&self) -> i8
    {
        self.machines.pop_count() as i8
    }
    fn num_workers(&self) -> i8
    {
        self.workers.pop_count() as i8
    }
}

fn merge_nodes(a: &Rc<RefCell<Node>>, b: &Rc<RefCell<Node>>)
{
    let mut node_ref = a.borrow_mut();
    {
        let worker_node = b.borrow();
        node_ref.workers |= worker_node.workers;
        node_ref.machines |= worker_node.machines;
    }
}

fn solve(workers: &[BitVec64]) -> usize
{
    let N = workers.len();

    //machines are 0..N, workers N..2*N
    let mut ds = DisjointSet::new(2 * N);

    for (w_idx, worker) in workers.iter().enumerate() {
        debug!(
            "Worker #{:>3}  machines {:0>width$b}  ",
            w_idx,
            worker.data,
            width = N
        );

        for machine in 0..N {
            if worker.get(machine) {
                ds.merge_sets(machine, w_idx + N);
            }
        }
    }

    debug!("Num sets: {}", ds.number_of_sets());

    let mut node_list: Vec<Rc<RefCell<Node>>> = (0..2 * N)
        .map(|i| {
            let mut node: Node = Default::default();
            if i >= N {
                node.workers.set(i - N, true);
            } else {
                node.machines.set(i, true);
            }
            Rc::new(RefCell::new(node))
        })
        .collect();

    let mut merged_node_list: Vec<Rc<RefCell<Node>>> = Vec::new();
    let mut in_merged = BitVec64::new();

    for node_idx in 0..2 * N {
        let merged_set = ds.get_repr(node_idx);

        if !in_merged.get(merged_set) {
            merged_node_list.push(node_list[merged_set].clone());
            in_merged.set(merged_set, true);
        }

        if !Rc::ptr_eq(&node_list[node_idx], &node_list[merged_set]) {
            merge_nodes(&node_list[merged_set], &node_list[node_idx]);

            debug!("Merged node {} to node {}", node_idx, merged_set);

            node_list[node_idx] = node_list[merged_set].clone();
        }
    }

    debug!("After merge\n");

    for machine in 0..N {
        debug!(
            "Machine #{:>3}  machines {:0>width$b} / workers {:0>width$b}  ",
            machine,
            node_list[machine].borrow().machines.data,
            node_list[machine].borrow().workers.data,
            width = N
        );
    }
    for worker in 0..N {
        let node = node_list[worker + N].borrow();
        debug!(
            "Worker  #{:>3}  machines {:0>width$b} / workers {:0>width$b} diff:{} ",
            worker,
            node.machines.data,
            node.workers.data,
            node.num_machines() - node.num_workers(),
            width = N
        );
    }

    for (idx, node) in merged_node_list.iter().enumerate() {
        //let node = node_list[worker + N].borrow();
        let node = node.borrow();
        debug!(
            "Node  #{:>3}  machines {:0>width$b} / workers {:0>width$b} diff:{} ",
            idx,
            node.machines.data,
            node.workers.data,
            node.num_machines() - node.num_workers(),
            width = N
        );
    }

    3
}
