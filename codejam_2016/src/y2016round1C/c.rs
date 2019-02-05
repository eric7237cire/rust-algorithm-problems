use codejam::util::codejam::run_cases;

use itertools::Itertools;
use std::io::Write;

use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::usize;

use permutohedron::LexicalPermutation;
/*

*/
pub fn solve_all_cases()
{
    run_cases(
        &[
            "C-small-practice",
            "C-large-practice"
        ],
        "y2016round1C",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let nums = reader.read_num_line();

                if case_no != 38 {
                   // continue;
                }

                println!("Solving case {}", case_no);

                writeln!(
                    buffer,
                    "Case #{}: {}",
                    case_no,
                    solve(nums[0], nums[1], nums[2], nums[3])
                )
                .unwrap();
            }
        },
    );
}

/*
fn calc(perms: &Vec< [usize;3] >, memo: &mut Vec<Vec<Vec<Vec<usize>>>>, K: usize, perm_index: usize) -> usize
{

}*/

fn solve(J: usize, P: usize, S: usize, K: usize) -> String
{
    println!("Starting J={} P={} S={}  K={}", J, P, S, K);

    

    let min_ks = min(K, S);

    let mut s_perm_list: HashSet<Vec<usize>> = HashSet::new();
    let mut s_perm: Vec<usize> = (1..=S).collect();

    loop {
        let mut v = s_perm[0..min_ks].to_vec();
        v.sort();
        s_perm_list.insert(v);

        if !s_perm.next_permutation() {
            break;
        }
    }

    let mut s_perm_list: Vec<Vec<usize>> = s_perm_list.iter().cloned().collect();
    s_perm_list.sort();
    let mut index = 0;
    let mut ans: Vec<[usize; 3]> = Vec::new();
    //s_perm = (1..=S).collect();

debug!("s_perm_list is {:?}", s_perm_list);

    

        let mut constraint_jp_count: HashMap<[usize; 2], usize> = HashMap::new();
    let mut constraint_js_count: HashMap<[usize; 2], usize> = HashMap::new();
    let mut constraint_ps_count: HashMap<[usize; 2], usize> = HashMap::new();

    let mut constraint_jps_count: HashSet<[usize; 3]> = HashSet::new();
ans.clear();
    

    for j in 1..=J {
        index = 0;
        for p in 1..=P {
            index += 1;
            for k in 0..min_ks {
                /*
                                let mut lowest_max = usize::MAX;
                                let mut lowest_min = usize::MAX;
                                let mut chosen_s = usize::MAX;
                                for s in 1..=S
                                {
                                    let max_cons = max(
                                    *constraint_js_count.get( &[j, s]).unwrap_or(&0),
                                    *constraint_ps_count.get( &[p,s] ).unwrap_or(&0));

                                    let min_cons = min(
                                    *constraint_js_count.get( &[j, s]).unwrap_or(&0),
                                    *constraint_ps_count.get( &[p,s] ).unwrap_or(&0));

                                    if constraint_jps_count.contains(&[j,p,s]) {
                                        continue;
                                    }

                                    if max_cons < lowest_max {
                                        chosen_s = s;
                                        lowest_max = max_cons;
                                        lowest_min = min_cons;
                                    } else if max_cons == lowest_max && min_cons < lowest_min {
                chosen_s = s;
                                        lowest_max = max_cons;
                                        lowest_min = min_cons;
                                    }
                                };

                                let item = [j, p, chosen_s];*/

                //let item = [j, p, s_perm[s_perm.len() - 1 -k]];
                let item = [j, p, s_perm_list[ (p-1) % s_perm_list.len() ][k]];
                debug!(
                    "Looking at {:?} index: {} s_perm_list: {:?} ",
                    item,
                    index,
                    s_perm_list[index % s_perm_list.len()]
                );

                let cons_count_1 = constraint_jp_count.entry([item[0], item[1]]).or_insert(0);

                //                assert!(*cons_count_1 < K);

                let cons_count_2 = constraint_js_count.entry([item[0], item[2]]).or_insert(0);

                /*              assert!(*cons_count_2 < K, format!("Used J={} S={} too many times K={}",
                item[0], item[2], K));*/

                let cons_count_3 = constraint_ps_count.entry([item[1], item[2]]).or_insert(0);

                /*
                assert!(*cons_count_3 < K, format!("Used P={} S={} too many times K={}",
                item[1], item[2], K));
                */

                *cons_count_1 += 1;
                *cons_count_2 += 1;
                *cons_count_3 += 1;

                if *cons_count_1 > K || *cons_count_2 > K || *cons_count_3 > K || 
                constraint_jps_count.contains(&item)
                {
                    //println!("Try another permutation");
                    //assert!(s_perm_list.next_permutation());
                    panic!("fail");
                }

                constraint_jps_count.insert(item);

                ans.push(item);
            }

            /*
            if !s_perm.next_permutation() {
                s_perm = (1..=S).collect();
            } */
        }

        /*if !s_perm_list.next_permutation() {
            s_perm_list.sort();
        }*/
        let first = s_perm_list.remove(0);
        s_perm_list.push(first);
    }


    format!(
        "{}\n{}",
        J * P * min_ks,
        //ans.len(),
        ans.iter().map(|p| p.iter().join(" ")).join("\n")
    )
}

fn solve_brute_force(J: usize, P: usize, S: usize, K: usize) -> String
{
    let mut perms: Vec<[usize; 3]> = Vec::new();
    for j in 1..=J {
        for p in 1..=P {
            for s in 1..=S {
                perms.push([j, p, s]);
            }
        }
    }

    if perms.len() > 18 {
        return "Too long".to_string();
    }

    assert!(perms.len() <= 27);

    println!("Starting J={} P={} S={}  K={}", J, P, S, K);

    let mut best_count = 0;
    let mut best_ans = String::new();

    //Now iterate over every subset
    'perms_loop: for subset in 0..1 << perms.len() {
        let mut count = 0;
        let mut constraint_jp_count: HashMap<[usize; 2], usize> = HashMap::new();
        let mut constraint_js_count: HashMap<[usize; 2], usize> = HashMap::new();
        let mut constraint_ps_count: HashMap<[usize; 2], usize> = HashMap::new();

        debug!(
            "Starting\n{}",
            perms
                .iter()
                .enumerate()
                .filter(|(p_idx, _)| subset >> p_idx & 1 > 0)
                .map(|(_, p)| p.iter().join(" "))
                .join("\n")
        );

        for (p_idx, p) in perms.iter().enumerate() {
            if subset >> p_idx & 1 == 0 {
                continue;
            }
            count += 1;

            let cons_count = constraint_jp_count.entry([p[0], p[1]]).or_insert(0);
            *cons_count += 1;

            if *cons_count > K {
                /*    debug!("Constraint count of [{},{}] {} > K {}",
                p[0], p[1], cons_count, K );*/
                continue 'perms_loop;
            }

            let cons_count = constraint_js_count.entry([p[0], p[2]]).or_insert(0);
            *cons_count += 1;

            if *cons_count > K {
                /*    debug!("Constraint count of [{},{}] {} > K {}",
                p[0], p[1], cons_count, K );*/
                continue 'perms_loop;
            }

            let cons_count = constraint_ps_count.entry([p[1], p[2]]).or_insert(0);
            *cons_count += 1;

            if *cons_count > K {
                /*    debug!("Constraint count of [{},{}] {} > K {}",
                p[0], p[1], cons_count, K );*/
                continue 'perms_loop;
            }
        }

        if count > best_count {
            println!("Found new best {}", count);
            best_count = count;
            best_ans = format!(
                "{}\n{}",
                best_count,
                perms
                    .iter()
                    .enumerate()
                    .filter(|(p_idx, _)| subset >> p_idx & 1 > 0)
                    .map(|(_, p)| p.iter().join(" "))
                    .join("\n")
            );
        }
    }

    best_ans
}
