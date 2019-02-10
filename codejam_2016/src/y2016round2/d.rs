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
use multiset::HashMultiSet;
use std::cell::RefCell;
//use std::cmp::max;
//use std::cmp::min;
//use std::collections::HashMap;
use std::i16;
use std::rc::Rc;

/*
Enumerating multisets
union find / disjoint set
rust RC/Refcell ; merging nodes from pointers
Much recursion
BitVec64 / bit manipulation

Main idea is that workers must enter as a square chunk, where
each worker knows each machine in the block
*/
pub fn solve_all_cases()
{
    run_cases(
        &["D-small-practice", "D-large-practice"],
        "y2016round2",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let N = reader.read_int();

                let workers: Vec<BitVec64> = (0..N)
                    .map(|_| reader.read_chars(N).into_iter().collect::<BitVec64>())
                    .collect();

                if case_no != 3 {
                    //  continue;
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
        (b.borrow().diff() + b.borrow().num_workers())
            .cmp(&(a.borrow().diff() + a.borrow().num_workers()))
    });

    for (idx, node) in merged_node_list.iter().enumerate() {
        //let node = node_list[worker + N].borrow();
        let node = node.borrow();
        debug!(
            "Merged Node  #{:>3}  machines {:0>width$b} / workers {:0>width$b} diff:{:>4} Num machines: {:>3} Num workers: {:>3} ",
            idx,
            node.machines.data,
            node.workers.data,
            node.diff(),
            node.num_machines(),
            node.num_workers(),
            width = N
        );
    }

    let mut multiset = HashMultiSet::new();

    for node in merged_node_list.iter() {
        let node = node.borrow();
        if node.diff() == 0 {
            continue;
        }
        let elem = MultiSetElement {
            num_workers: node.num_workers(),
            diff: node.diff(),
        };
        multiset.insert(elem);
    }

    let mut distinct = multiset
        .distinct_elements()
        .cloned()
        .collect::<Vec<MultiSetElement>>();

    distinct.sort_by(|a, b| {
        a.diff
            .cmp(&b.diff)
            .then_with(|| a.num_workers.cmp(&b.num_workers))
    });

    let elem_counts: Vec<i16> = distinct
        .iter()
        .map(|mse| multiset.count_of(mse) as i16)
        .collect();

    let mut cur_elem_counts = vec![0i16; elem_counts.len()];
    let mut ans = Vec::new();

    enumerate_subsets(
        &distinct[..],
        &elem_counts[..],
        0,
        0,
        &mut cur_elem_counts,
        &mut ans,
    );

    debug!("NUMBER OF SUBSETS: {}", ans.len());

    for (idx, d) in distinct.iter().enumerate() {
        debug!(
            "Distinct non zero diff node #{}: diff={} worker_count={} elem count:{}",
            idx, d.diff, d.num_workers, elem_counts[idx]
        );
    }

    //We want to remove subsets that fully contain other subsets
    let mut filtered_subsets = Vec::new();
    for idx1 in 0..ans.len() {
        let mut ok = true;
        for idx2 in 0..ans.len() {
            if idx2 == idx1 {
                continue;
            }
            if ans[idx2].iter().zip(ans[idx1].iter()).all(|(b, a)| b <= a) {
                ok = false;
                break;
            }
        }

        if ok {
            filtered_subsets.push(ans[idx1].clone());
        }
    }

    /*
    for (idx, ss) in ans.iter().enumerate() {
        debug!("Ans #{}: {:?}", idx, ss);
    }*/

    for (idx, ss) in filtered_subsets.iter().enumerate() {
        debug!("Subset #{}: {:?}", idx, ss);
    }

    let mut cur_chosen_zero_subsets = Vec::new();
    let mut chosen_zero_subsets = Vec::new();

    let mut min_workers = i16::MAX;

    best_subsets(
        &distinct[..],
        &filtered_subsets[..],
        0,
        &mut elem_counts.clone(),
        &mut cur_chosen_zero_subsets,
        &mut chosen_zero_subsets,
        &mut min_workers,
    );

    debug!(
        "Best subsets: {:?} counts: {:?}",
        chosen_zero_subsets, elem_counts
    );

    let mut used_node = BitVec64::new();
    let mut has_been_chosen = BitVec64::new();

    for chosen_ss_idx in chosen_zero_subsets.iter() {
        let ss = &filtered_subsets[*chosen_ss_idx];

        let mut vec_node_list = Vec::new();

        for (distinct_idx, count) in ss.iter().enumerate() {
            if *count == 0 {
                continue;
            }
            let distinct_node = &distinct[distinct_idx];

            for c in 0..*count {
                for idx in 0..merged_node_list.len() {
                    if has_been_chosen.get(idx) {
                        continue;
                    }

                    let node = merged_node_list[idx].borrow();

                    if node.num_workers() == distinct_node.num_workers
                        && node.diff() == distinct_node.diff
                    {
                        debug!(
                            "Found node #{} matching distinct node #{} count {}
workers = {} diff = {}",
                            idx, distinct_idx, c, distinct_node.num_workers, distinct_node.diff
                        );

                        vec_node_list.push(idx);
                        has_been_chosen.set(idx, true);
                        break;
                    }
                }
            }
        }

        assert_eq!(
            vec_node_list.len(),
            ss.iter().cloned().sum::<i16>() as usize
        );

        debug!("Merging {:?}", vec_node_list);

        let first_node_idx = vec_node_list[0];
        for &node_idx in vec_node_list.iter().skip(1) {
            assert!(!Rc::ptr_eq(
                &merged_node_list[node_idx],
                &merged_node_list[first_node_idx]
            ));

            merge_nodes(
                &merged_node_list[first_node_idx],
                &merged_node_list[node_idx],
            );

            debug!("Merged node {} to node {}", node_idx, first_node_idx);

            merged_node_list[node_idx] = merged_node_list[first_node_idx].clone();

            used_node.set(node_idx, true);
        }
    }

    let mut cost: i16 = 0;

    for idx in 0..merged_node_list.len() {
        if used_node.get(idx) {
            continue;
        }

        assert_eq!(0, merged_node_list[idx].borrow().diff());

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

    }

    cost
}

#[derive(Eq, Hash, PartialEq, Clone)]
struct MultiSetElement
{
    num_workers: i16,
    diff: i16,
}

///Finds subsets of sum 0
fn enumerate_subsets(
    elements: &[MultiSetElement],
    elem_counts: &[i16],
    current_index: usize,
    current_diff: i16,

    current_elem_counts: &mut Vec<i16>,
    ans: &mut Vec<Vec<i16>>,
)
{
    if current_index == elem_counts.len() {
        if current_diff == 0 && current_elem_counts.iter().any(|c| c > &0) {
            ans.push(current_elem_counts.clone());
        }
        return;
    }

    for count in 0..=elem_counts[current_index] {
        current_elem_counts[current_index] = count;

        let next_current_diff = current_diff + count * elements[current_index].diff;

        enumerate_subsets(
            elements,
            elem_counts,
            1 + current_index,
            next_current_diff,
            current_elem_counts,
            ans,
        );
    }
}

///Chooses zero-sum subsets with lowest sq worker count
fn best_subsets(
    elements: &[MultiSetElement],
    //list of how many of each element
    subsets: &[Vec<i16>],
    current_index: usize,
    remaining_elements: &mut Vec<i16>,

    cur_ans: &mut Vec<usize>,

    ans: &mut Vec<usize>,
    min_sq_worker_count: &mut i16,
)
{
    if current_index == subsets.len() {
        if remaining_elements.iter().all(|re| *re == 0) {
            let worker_sq_count: i16 = cur_ans
                .iter()
                .map(|ss_idx| {
                    let ss_worker_count = subsets[*ss_idx]
                        .iter()
                        .enumerate()
                        .map(|(idx, count)| count * elements[idx].num_workers)
                        .sum::<i16>();

                    ss_worker_count * ss_worker_count
                })
                .sum();

            if worker_sq_count < *min_sq_worker_count {
                *min_sq_worker_count = worker_sq_count;
                ans.clear();
                ans.extend(cur_ans.iter());
                debug!("Min square count is {}", worker_sq_count);
            }
        }
        return;
    }

    best_subsets(
        elements,
        subsets,
        1 + current_index,
        remaining_elements,
        cur_ans,
        ans,
        min_sq_worker_count,
    );

    let can_use = remaining_elements
        .iter()
        .zip(subsets[current_index].iter())
        .all(|(re, ss)| re >= ss);

    if can_use {
        for (re, ss) in remaining_elements
            .iter_mut()
            .zip(subsets[current_index].iter())
        {
            *re -= *ss;
            assert!(*re >= 0);
        }
        cur_ans.push(current_index);

        //We also want to try to take multiple instances
        best_subsets(
            elements,
            subsets,
            current_index,
            remaining_elements,
            cur_ans,
            ans,
            min_sq_worker_count,
        );

        for (re, ss) in remaining_elements
            .iter_mut()
            .zip(subsets[current_index].iter())
        {
            *re += *ss;
            assert!(*re >= 0);
        }
        cur_ans.pop();
    }
}
