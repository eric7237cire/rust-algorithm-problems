
pub fn solve_small(home: &Point, dest: &Point, teleporters: &Vec<Point>) -> Option<u64>
{
    let target_distance = dist(home, dest);
    //let mut L: Vec<Vec<i64>> = Vec::new();
    //let mut U: Vec<Vec<i64>> = Vec::new();

    let mut initial = Vec::new();
    for t in teleporters.iter() {
        initial.push( dist(home, t) );
    }
    let mut L = initial.clone();
    let mut U = initial.clone();

    /*
    By definition, Lt,i+1 and Ut,i+1 are the distances from t to its closest and farthest points in Ri, respectively.
     The farthest point in Ri from t is at a distance which is the maximum over all teleporters u of dist(t, u) + Uu,i 
     (this is the distance to the point on the surface of the sphere centered at u with radius Uu,
     i that is the opposite direction from t).
    */
    for i in 1..10000
    {
        let mut new_L = Vec::new();
        let mut new_U = Vec::new();

        for (t_idx, t) in teleporters.iter().enumerate() {

            if dist(&dest, t) >= L[t_idx] &&
               dist(&dest, t) <= U[t_idx] {
                return Some(i);
            }
       
            if teleporters.len()==1 {
                return None;
            }

            let mut low = None;
            let mut high = None;    
            for (u_idx, u) in teleporters.iter().enumerate()
            {
                if u_idx == t_idx {
                    continue;
                }
                //Greatest distance from teleporter u + distance of t to u; 
                //this is the furthest one could teleport using teleporter t 
                let maybe_high = U[u_idx] + dist(u, t);
                if high.is_none() || maybe_high > high.unwrap() {
                    high = Some(maybe_high);
                    
                }

                /*
                . For each teleporter u we need to consider:

dist(t, u) - Uu,i if dist(t, u) > Uu,i (t is outside the outer sphere centered at u),
Lu,i - dist(t, u) if dist(t, u) < Lu,i (t is inside the inner sphere), or
0, in all other cases (t is in between, that is, it is itself a reachable point).
*/

                let dist_tu = dist(t,u);
                let maybe_low = if dist_tu > U[u_idx] {
                    //lowest distance is outside the outer sphere
                    dist_tu - U[u_idx]
                } else if dist_tu < L[u_idx] {
                    //teleport to lower sphere
                    L[u_idx] - dist_tu 
                } else {
                    0 
                };

                if low.is_none() || maybe_low < low.unwrap() {
                    low = Some(maybe_low);
                }
            }


            new_L.push(low.unwrap());
            new_U.push(high.unwrap());
        }

        mem::swap(&mut L, &mut new_L);
        mem::swap(&mut U, &mut new_U);


    }

    None 
}



/// Tests large observation that we only need to calculate U
/// And validates iterative squaring approach to calculing max U distance per step

fn solve_small_only_u(home: &Point, dest: &Point, teleporters: &Vec<Point>) -> Option<u64>
{
    //let mut L: Vec<Vec<i64>> = Vec::new();
    //let mut U: Vec<Vec<i64>> = Vec::new();

    //let target_distance = dist(home, dest);

    let min_dist_home = teleporters
        .iter()
        .fold(i64::MAX, |acc, t| min(acc, dist(&home, t)));

    let min_dist_dest = teleporters
        .iter()
        .fold(i64::MAX, |acc, t| min(acc, dist(&dest, t)));

    let max_dist_home = teleporters
        .iter()
        .fold(i64::MIN, |acc, t| max(acc, dist(&home, t)));

    let max_dist_dest = teleporters
        .iter()
        .fold(i64::MIN, |acc, t| max(acc, dist(&dest, t)));

    let max_dist = max(max_dist_dest, max_dist_home);

    //extra
    let mut dist_matrix = Vec::new();

    for steps_idx in 0..50 {
        if (1i64 << steps_idx) > max_dist {
            break;
        }
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

        println!(
            "After step idx {} max is {}",
            steps_idx,
            dist_matrix[steps_idx].iter().flatten().max().unwrap()
        );
    }

    println!("min. d home {} dest {}", min_dist_home, min_dist_dest);

    let (home, dest) = if min_dist_home > min_dist_dest {
        (dest, home)
    } else {
        (home, dest)
    };

    for t in teleporters.iter() {
        //Check if one teleport is enough
        if dist(home, t) == dist(dest, t) {
            return Some(1);
        }
    }

    let mut initial = Vec::new();
    for (_t_idx, t) in teleporters.iter().enumerate() {
        /*println!("Teleporter #{}, dist home: {}",
        t_idx, dist(home, t));*/
        initial.push(dist(home, t));
    }

    let mut l = initial.clone();
    let mut u = initial.clone();

    /*
    By definition, Lt,i+1 and Ut,i+1 are the distances from t to its closest and farthest points in Ri, respectively.
     The farthest point in Ri from t is at a distance which is the maximum over all teleporters u of dist(t, u) + Uu,i
     (this is the distance to the point on the surface of the sphere centered at u with radius Uu,
     i that is the opposite direction from t).
    */
    for i in 1..10000 {
        if i < 68 {
            println!("i {} max is {}", i, u.iter().max().unwrap());
        }

        let fast_umax = get_longest_path_for_step(&dist_matrix, &initial, i - 1);

        if i > 1 {
            let fast_umax_all = fast_umax.iter().max().unwrap();
            let current_umax = u.iter().max().unwrap();

            assert_eq!(*current_umax + 1, *fast_umax_all);

            /*println!("maxes: {} and {}\nU vs fast: {:?} ",
            fast_umax_all,current_umax, fast_umax.iter().zip(U.iter()).collect::<Vec<_>>());
            */
        }

        let mut new_l = Vec::new();
        let mut new_u = Vec::new();

        for (t_idx, t) in teleporters.iter().enumerate() {
            if i > 1 {
                let current_umax = u[t_idx];
                let fast_umax_t = fast_umax[t_idx];
                assert_eq!(current_umax, fast_umax_t);
            }

            if
            //dist(&dest, t) >= L[t_idx] &&
            i > 1 && dist(&dest, t) <= u[t_idx] {
                return Some(i as u64);
            }

            if teleporters.len() == 1 {
                return None;
            }

            /*           println!("Starting iteration #{}, teleporter #{} U[{}] = {}",
            i, t_idx, t_idx, U[t_idx]);*/

            let mut low = None;
            let mut high = None;
            for (u_idx, u) in teleporters.iter().enumerate() {
                if u_idx == t_idx {
                    continue;
                }
                //Greatest distance from teleporter u + distance of t to u;
                //this is the furthest one could teleport using teleporter t
                let maybe_high = u[u_idx] + dist(u, t);
                if high.is_none() || maybe_high > high.unwrap() {
                    high = Some(maybe_high);
                }

                /*
                                . For each teleporter u we need to consider:

                dist(t, u) - Uu,i if dist(t, u) > Uu,i (t is outside the outer sphere centered at u),
                Lu,i - dist(t, u) if dist(t, u) < Lu,i (t is inside the inner sphere), or
                0, in all other cases (t is in between, that is, it is itself a reachable point).
                */

                let dist_tu = dist(t, u);
                let maybe_low = if dist_tu > u[u_idx] {
                    //lowest distance is outside the outer sphere
                    dist_tu - u[u_idx]
                } else if dist_tu < l[u_idx] {
                    //teleport to lower sphere
                    l[u_idx] - dist_tu
                } else {
                    0
                };

                if low.is_none() || maybe_low < low.unwrap() {
                    low = Some(maybe_low);
                }
            }

            new_l.push(low.unwrap());
            new_u.push(high.unwrap());
        }

        mem::swap(&mut l, &mut new_l);
        mem::swap(&mut u, &mut new_u);
    }

    None
}
