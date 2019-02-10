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

//use codejam::algo::graph::disjointset::DisjointSet;
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

                let workers: Vec<BitVec64> =
                    (0..N).map(|_| reader.read_chars(N).into_iter().collect::<BitVec64>()).collect();

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
    workers: BitVec64
}

impl Drop for Node {
    fn drop(&mut self) {
        debug!("Node dropped machines {:b} workers {:b}", self.machines, self.workers);
    }
}

impl Node
{
    fn num_machines(&self) -> i8 {
        self.machines.pop_count() as i8
    }
    fn num_workers(&self) -> i8 {
        self.workers.pop_count() as i8
    }
}

fn solve(workers: &[BitVec64]) -> usize
{
    let N = workers.len();

    let mut machine_nodes: Vec<Rc<RefCell<Node>>> = (0..N).map(|m| {
        let mut node:Node = Default::default();
        node.machines.set(m, true);
        Rc::new(RefCell::new(node))
    }).collect();


    for machine in 0..N {
        debug!("Machine #{:>3}  machines {:0>width$b} / workers {:0>width$b}  ", machine,
               machine_nodes[machine].borrow().workers.data,
        machine_nodes[machine].borrow().machines.data,
            width=N
        );
    }

    let mut worker_nodes: Vec<Rc<RefCell<Node>>> = (0..N).map(|w| {
        let mut node:Node = Default::default();
        node.workers.set(w, true);
        Rc::new(RefCell::new(node))
    }).collect();

    for (w_idx, worker) in workers.iter().enumerate() {
        let mut first_machine = N;
        for machine in 0..N {
            if worker.get(machine) {
                if first_machine == N {
                    first_machine = machine;
                } else {
                    if !Rc::ptr_eq(&machine_nodes[first_machine],
                        &machine_nodes[machine]) {
                        machine_nodes[machine] = machine_nodes[first_machine].clone();
                    }
                }



                if !Rc::ptr_eq(&machine_nodes[machine], &worker_nodes[w_idx]) {
                    let mut node_ref = machine_nodes[machine].borrow_mut();
                    {
                        let worker_node = worker_nodes[w_idx].borrow();
                        node_ref.workers |= worker_node.workers;
                        node_ref.machines |= worker_node.machines;
                    }
                    worker_nodes[w_idx] = machine_nodes[machine].clone();


                }
                //ds.merge_sets(first_machine, machine);
            }
        }
    }

    debug!("After workers\n");

    for machine in 0..N {
        debug!("Machine #{:>3}  machines {:0>width$b} / workers {:0>width$b}  ", machine,
               machine_nodes[machine].borrow().workers.data,
        machine_nodes[machine].borrow().machines.data,
            width=N
        );
    }
    for worker in 0..N {
        debug!("Worker  #{:>3}  machines {:0>width$b} / workers {:0>width$b}  ", worker,
               worker_nodes[worker].borrow().workers.data,
        worker_nodes[worker].borrow().machines.data,
            width=N
        );
    }

   //let worker_nodes = (0..N).map(|_|)

    /*
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

    let mut machine_in_node = BitVec64::new();

    //Build at most one node per machine
    let mut nodes = Vec::new();

    for m in 0..N {
        if machine_in_node.getb(m) {
            continue;
        }





        for n in m+1..N {

        }
    }*/



    3
}
