use codejam::util::codejam::run_cases;
use codejam::util::input::*;

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
                let (_, P) = reader.read_tuple_2::<usize>();
                let mut G: Vec<_> = reader.read_num_line::<usize>();

                if case_no != 1 {
                    //        continue;
                }

                println!("Solving case {}", case_no);

                writeln!(buffer, "Case #{}: {:.6}", case_no, solve(&mut G, P)).unwrap();
            }
        },
    );
}

fn solve(G: &mut Vec<usize>, P: usize) -> String
{
    for g in G.iter_mut() {
        *g %= P;
    }

    let mut G_count = (0..P)
        .map(|i| G.iter().filter(|&&g| g == i).count())
        .collect::<Vec<_>>();

    debug!("P={} G_count={:?}", P, G_count);

    //groups are listed by their mod, but we need a 2nd index with how many leftovers they can consume
    let NEED_INDEX = (0..P).map(|g| (P - g) % P).collect::<Vec<_>>();
    let mut leftover = 0usize;
    let mut groups_happy = 0;
    'outer: for _ in 0..G.len() {
        //state machine
        if leftover == 0 {
            groups_happy += 1;

            //maintaining 0 state is important, so we need to short circuit any cases
            //where we have the group than can consume 100% of the leftovers
            for (need_idx, &g_idx) in NEED_INDEX.iter().enumerate() {
                if G_count[g_idx] > 0 && G_count[NEED_INDEX[(P - need_idx) % P]] > 0 {
                    G_count[g_idx] -= 1;
                    leftover = (P - need_idx) % P;
                    continue 'outer;
                }
            }
        }

        //if we can make leftovers 0, then do it
        if G_count[NEED_INDEX[leftover]] > 0 {
            G_count[NEED_INDEX[leftover]] -= 1;
            leftover = 0;
            continue;
        }

        //fall through case, just take anything
        for (need_idx, &g_idx) in NEED_INDEX.iter().enumerate() {
            if G_count[g_idx] > 0 {
                G_count[g_idx] -= 1;
                leftover = (leftover + P - need_idx) % P;
                break;
            }
        }
    }

    format!("{}\n", groups_happy)
}
