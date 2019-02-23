use codejam::util::codejam::run_cases;
use std::io::Write;

/*
state machine
custom algorithm / proof
optimization
*/
pub fn solve_all_cases()
{
    run_cases(
        &["A-small-practice", "A-large-practice"],
        "y2017round2",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let (_, p) = reader.read_tuple_2::<usize>();
                let mut g: Vec<_> = reader.read_num_line::<usize>();

                if case_no != 1 {
                    //        continue;
                }

                println!("Solving case {}", case_no);

                writeln!(buffer, "Case #{}: {}", case_no, solve(&mut g, p)).unwrap();
            }
        },
    );
}

fn solve(g: &mut Vec<usize>, p: usize) -> i32
{
    for g in g.iter_mut() {
        *g %= p;
    }

    let mut g_count = (0..p)
        .map(|i| g.iter().filter(|&&g| g == i).count())
        .collect::<Vec<_>>();

    debug!("P={} G_count={:?}", p, g_count);

    //groups are listed by their mod, but we need a 2nd index with how many leftovers they can consume
    let need_index = (0..p).map(|g| (p - g) % p).collect::<Vec<_>>();
    let mut leftover = 0usize;
    let mut groups_happy = 0;
    'outer: for _ in 0..g.len() {
        //state machine
        if leftover == 0 {
            groups_happy += 1;

            //maintaining 0 state is important, so we need to short circuit any cases
            //where we have the group than can consume 100% of the leftovers
            for (need_idx, &g_idx) in need_index.iter().enumerate() {
                if g_count[g_idx] > 0 && g_count[need_index[(p - need_idx) % p]] > 0 {
                    g_count[g_idx] -= 1;
                    leftover = (p - need_idx) % p;
                    continue 'outer;
                }
            }
        }

        //if we can make leftovers 0, then do it
        if g_count[need_index[leftover]] > 0 {
            g_count[need_index[leftover]] -= 1;
            leftover = 0;
            continue;
        }

        //fall through case, just take anything
        for (need_idx, &g_idx) in need_index.iter().enumerate() {
            if g_count[g_idx] > 0 {
                g_count[g_idx] -= 1;
                leftover = (leftover + p - need_idx) % p;
                break;
            }
        }
    }

     groups_happy
}
