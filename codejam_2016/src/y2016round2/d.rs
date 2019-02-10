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
use std::cmp::max;
use std::cmp::min;
use std::i16;
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

                if case_no != 43 {
                   // continue;
                }

                println!("Solving case {}", case_no);

                writeln!(
                    buffer,
                    "Case #{}: {}",
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
    fn num_machines(&self) -> i16
    {
        self.machines.pop_count() as i16
    }
    fn num_workers(&self) -> i16
    {
        self.workers.pop_count() as i16
    }
    fn diff(&self) -> i16
    {
        self.num_machines() - self.num_workers()
    }
    fn size(&self) -> i16
    {
        max(self.num_machines(), self.num_workers())
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

fn solve(workers: &[BitVec64]) -> i16
{
    let N = workers.len();
    let Ni16 = N as i16;

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

    merged_node_list.sort_by(|a, b| {
        max(b.borrow().num_machines(), b.borrow().num_workers())
            .cmp(&max(a.borrow().num_machines(), a.borrow().num_workers()))
    });

    for (idx, node) in merged_node_list.iter().enumerate() {
        //let node = node_list[worker + N].borrow();
        let node = node.borrow();
        debug!(
            "Merged Node  #{:>3}  machines {:0>width$b} / workers {:0>width$b} diff:{} ",
            idx,
            node.machines.data,
            node.workers.data,
            node.num_machines() - node.num_workers(),
            width = N
        );
    }

    let mut used_node = BitVec64::new();

    let mut cost: i16 = 0;

    for idx in 0..merged_node_list.len() {
        if used_node.get(idx) {
            continue;
        }

        if merged_node_list[idx].borrow().diff() == 0 {
            let node = merged_node_list[idx].borrow();
            cost += node.num_machines() * node.num_workers()
                - (0..N)
                    .map(|w| {
                        if node.workers.get(w) {
                            workers[w].pop_count() as i16
                        } else {
                            0
                        }
                    })
                    .sum::<i16>();
            used_node.set(idx, true);
        } else {
            debug!("find a subset for merged node #{}", idx);

            //dp[ node_idx ][sum + N] = cheapest that sums to x using nodes up to node_idx
            let NON_INIT = 5000i16;
            let mut dp = vec![vec![NON_INIT; 2 * N + 2]; 1];
            for dp_idx in 0..merged_node_list.len() {
                let dp_node = merged_node_list[dp_idx].borrow();

                
                dp.push(dp[dp_idx].clone());

                let merged_node_index = dp_idx;
                //Because we have an initial dp vector with max sizes
                let dp_idx = dp_idx + 1;
                
                assert_eq!(dp_idx+1, dp.len());

                if dp_node.diff() == 0 {
                    continue;
                }

                if used_node.get(merged_node_index) {
                    continue;
                }

                if merged_node_index == idx {
                    continue;
                }

                debug!("Looping through {} to= {}",
                max(-Ni16, -Ni16 + dp_node.diff()), min(Ni16, Ni16 + dp_node.diff())
                );

                for val in max(-Ni16, -Ni16 + dp_node.diff())..=min(Ni16, Ni16 + dp_node.diff()) {
                    dp[dp_idx][(val + Ni16) as usize] = min(
                        dp[dp_idx - 1][(val + Ni16) as usize],
                        dp[dp_idx - 1][(val - dp_node.diff() + Ni16) as usize] + dp_node.size(),
                    );
                }

                dp[dp_idx][(dp_node.diff() + Ni16) as usize] =
                    min(dp[dp_idx][(dp_node.diff() + Ni16) as usize], dp_node.size());
            }

            let mut chosen_subset = Vec::new();
            let mut last_element_idx = merged_node_list.len();

            let mut target_diff = {
                let node = merged_node_list[idx].borrow();
                 - node.diff()
            };

            let mut optimal_size = 
                dp[ last_element_idx ][ (Ni16 + target_diff) as usize ];
        

            assert_ne!(NON_INIT, optimal_size);
            
            'dp_path_loop: loop
            {
                //Find first element with the optimal size              
                for dp_idx in 1..=merged_node_list.len() {
                    let value = dp[dp_idx][(Ni16 + target_diff) as usize];
                    debug!("For up to merged node {} for sum of {} smallest size is {}. ",
                    dp_idx-1, target_diff, value);
                    
                    if value == optimal_size {
                        chosen_subset.push(dp_idx-1);

                        let node =  merged_node_list[dp_idx-1].borrow();
                        if node.size() == optimal_size {
                            break 'dp_path_loop; 
                        }

                        optimal_size = optimal_size - node.size();
                        assert!(optimal_size > 0);
                        target_diff = target_diff - node.diff();

                        assert!( dp_idx <= last_element_idx );
                        last_element_idx = dp_idx;
                        
                        break;
                    }
                    
                }
            }

            debug!("Merging all the chosen nodes");
            for chosen in chosen_subset {
                used_node.set(chosen, true);
                assert_ne!(chosen, idx);
                assert!(!Rc::ptr_eq(
                    &merged_node_list[idx],
                    &merged_node_list[chosen]
                ));

                merge_nodes(&merged_node_list[idx], &merged_node_list[chosen]);

                debug!("Chosen node {} to node {}", chosen, idx);

                merged_node_list[chosen] = merged_node_list[idx].clone();
            }

            let node = merged_node_list[idx].borrow();

            cost += node.num_machines() * node.num_workers()
                - (0..N)
                    .map(|w| {
                        if node.workers.get(w) {
                            workers[w].pop_count() as i16
                        } else {
                            0
                        }
                    })
                    .sum::<i16>();
            used_node.set(idx, true);
        }
    }

    cost
}
