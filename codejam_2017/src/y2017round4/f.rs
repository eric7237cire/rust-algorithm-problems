use codejam::util::codejam::run_cases;
use std::cmp::{max, min};
use std::i64;
use std::io::Write;
use std::mem;

/*
*/

pub fn solve_all_cases()
{
    run_cases(
        &["F-small-practice", "F-large-practice"],
        "y2017round4",
        |reader, buffer| {
            let t = reader.read_int();

            for case in 1..=t {
                let n = reader.read_int::<usize>();
                let home = reader.read_array_3::<i64>();
                let dest = reader.read_array_3::<i64>();

                let teleporters: Vec<_> = (0..n).map(|_| reader.read_array_3::<i64>()).collect();

                if case != 22 {
                    //continue;

                }

                println!("Solving {}", case);

                //if teleporters.len() > 2 && teleporters.len() <= 6 {

                writeln!(
                    buffer,
                    "Case #{}: {}",
                    case,
                    //if let Some(ans) = solve_small_only_U(&home, &dest, &teleporters) {
                    if let Some(ans) = solve(&home, &dest, &teleporters) {
                        format!("{}", ans)
                    } else {
                        "IMPOSSIBLE".to_string()
                    }
                )
                .unwrap();
                //}
            }
        },
    );
}

type Point = [i64; 3];

fn dist(a: &Point, b: &Point) -> i64
{
    (a[0] - b[0]).abs() + (a[1] - b[1]).abs() + (a[2] - b[2]).abs()
}

fn get_longest_path_for_step(
    dist_matrix: &Vec<Vec<Vec<i64>>>,
    home_dist: &Vec<i64>,
    steps: usize,
) -> Vec<i64>
{
    let n = dist_matrix[0].len();

    let mut ans = vec![vec![-1; n]; n];
    let mut new_ans = vec![vec![-1; n]; n];

    for step_idx in 0..dist_matrix.len() {
        if (1 << step_idx) & steps == 0 {
            continue;
        }

        if ans[0][0] == -1 {
            //println!("Initializing with {}", step_idx);
            ans = dist_matrix[step_idx].clone();
            continue;
        }

        // println!("Multiplying with {}", step_idx);

        //
        for t1_idx in 0..n {
            for t2_idx in 0..n {
                let mut best = -1;
                for v_idx in 0..n {
                    best = max(
                        best,
                        ans[t1_idx][v_idx] + dist_matrix[step_idx][v_idx][t2_idx],
                    );
                }

                new_ans[t1_idx][t2_idx] = best;

                /* println!("Dist matrix {} to {}, step {} = {}",
                t1_idx, t2_idx, steps_idx, best); */
            }
        }

        mem::swap(&mut ans, &mut new_ans);
    }

    /* (0..N).map(
    |t_idx| ans[t_idx].iter().max().unwrap() +
    home_dist[t_idx]).collect()*/

    //return indexed by end point
    (0..n)
        .map(|stop_idx| {
            { (0..n).map(|start_idx| ans[start_idx][stop_idx] + home_dist[start_idx]) }
                .max()
                .unwrap()
        })
        .collect()
}

fn solve(home: &Point, dest: &Point, teleporters: &Vec<Point>) -> Option<u64>
{
    let min_dist_home = teleporters
        .iter()
        .fold(i64::MAX, |acc, t| min(acc, dist(&home, t)));

    let min_dist_dest = teleporters
        .iter()
        .fold(i64::MAX, |acc, t| min(acc, dist(&dest, t)));

    //extra
    let mut dist_matrix = Vec::new();

    /*
    create a matrix [log steps][t_idx][t2_idx]

    calculates dist[32 steps][t1][t2] =
    best of dist[16 steps][t1][v] +
    dist[16 steps][v][t2]
    */

    for steps_idx in 0..44 {
        dist_matrix.push(vec![vec![-1; teleporters.len()]; teleporters.len()]);

        if steps_idx == 0 {
            for (t1_idx, t1) in teleporters.iter().enumerate() {
                for (t2_idx, t2) in teleporters.iter().enumerate() {
                    dist_matrix[steps_idx][t1_idx][t2_idx] = dist(t1, t2);
                }
            }
        } else {
            for (t1_idx, _t1) in teleporters.iter().enumerate() {
                for (t2_idx, _t2) in teleporters.iter().enumerate() {
                    let mut best = -1;
                    for (v_idx, _v) in teleporters.iter().enumerate() {
                        best = max(
                            best,
                            dist_matrix[steps_idx - 1][t1_idx][v_idx]
                                + dist_matrix[steps_idx - 1][v_idx][t2_idx],
                        );
                    }

                    dist_matrix[steps_idx][t1_idx][t2_idx] = best;

                    /* println!("Dist matrix {} to {}, step {} = {}",
                    t1_idx, t2_idx, steps_idx, best); */
                }
            }
        }

        let u_max = dist_matrix[steps_idx].iter().flatten().max().unwrap();

        // take care of overflow
        if *u_max >= (1i64 << 50) {
            break;
        }

        /* println!("In precalculations: After step idx {} max is {}",
        steps_idx, u_max);*/
    }

    //println!("min. d home {} dest {}", min_dist_home, min_dist_dest);

    //make home the closest one
    let (home, dest) = if min_dist_home > min_dist_dest {
        (dest, home)
    } else {
        (home, dest)
    };

    //Check if one teleport is enough, after this code only works with 2
    for t in teleporters.iter() {
        if dist(home, t) == dist(dest, t) {
            return Some(1);
        }
    }

    let mut dist_home = Vec::new();
    let mut dist_target = Vec::new();

    for t in teleporters.iter() {
        //Check if one teleport is enough
        if dist(home, t) == dist(dest, t) {
            return Some(1);
        }
        dist_home.push(dist(home, t));
        dist_target.push(dist(dest, t));
    }

    //only case where its impossible
    if teleporters.len() == 1 {
        return None;
    }

    let mut min_num_steps = 1;
    let mut max_num_steps = 1 << (dist_matrix.len() - 1);

    while max_num_steps > min_num_steps {
        let steps = (max_num_steps + min_num_steps) / 2;

        let fast_umax = get_longest_path_for_step(&dist_matrix, &dist_home, steps);

        /*
        println!("Steps {} min {} max {} umax {}\n{:?}\nDist target:\n{:?}", steps,
         min_num_steps,
         max_num_steps,
         fast_umax.iter().max().unwrap(),
         fast_umax,
         dist_target
         );*/

        let any_in_range =
            (0..teleporters.len()).any(|t_idx| dist_target[t_idx] <= fast_umax[t_idx]);

        if !any_in_range {
            min_num_steps = steps + 1;
        } else {
            max_num_steps = steps;
        }
    }

    Some(1 + max_num_steps as u64)
}

