use bit_set::BitSet;
use bit_vec::BitVec;
use codejam::util::codejam::run_cases;
use itertools::Itertools;
use std::io::Write;

/*
BitSets & BitVecs
*/
pub fn solve_all_cases()
{
    run_cases(
        &["B-small-practice", "B-large-practice"],
        "y2008round1a",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let num_flavors = reader.read_int();
                let num_customers = reader.read_int();

                let cust_data: Vec<Vec<usize>> =
                    (0..num_customers).map(|_| reader.read_num_line()).collect();

                if case_no != 3 {
                    // continue;
                }

                println!("Solving case {}", case_no);

                writeln!(
                    buffer,
                    "Case #{}: {}",
                    case_no,
                    if let Some(ans) = solve(num_flavors, cust_data.as_slice()) {
                        ans.iter().map(|b| if b { '1' } else { '0' }).join(" ")
                    } else {
                        "IMPOSSIBLE".to_string()
                    }
                )
                .unwrap();
            }
        },
    );
}

fn solve(num_flavors: usize, cust_data: &[Vec<usize>]) -> Option<BitVec>
{
    let mut malted_prefs: Vec<Option<usize>> = vec![None; cust_data.len()];
    let unmalted_prefs: Vec<BitSet> = cust_data
        .into_iter()
        .enumerate()
        .map(|(c_idx, v)| {
            let t = v[0];
            assert_eq!(t * 2 + 1, v.len());
            let mut bs = BitSet::with_capacity(t);
            //make everything 1 based
            for e in v[1..].chunks_exact(2) {
                if e[1] == 1 {
                    assert!(malted_prefs[c_idx].is_none());
                    malted_prefs[c_idx] = Some(e[0] - 1);
                } else {
                    assert_eq!(0, e[1]);
                    bs.insert(e[0] - 1);
                }
            }
            bs
        })
        .collect();

    let mut unmalted = BitSet::with_capacity(num_flavors);

    //start with everything malted
    for f in 0..num_flavors {
        unmalted.insert(f);
    }

    let mut choice_made = true;

    while choice_made {
        choice_made = false;

        for (c_idx, unmalted_pref) in unmalted_prefs.iter().enumerate() {
            //Is this customer satisfied with one of the milkshakes?
            if let Some(mc) = malted_prefs[c_idx] {
                if !unmalted.contains(mc) {
                    continue;
                }
            }

            if !unmalted.is_disjoint(unmalted_pref) {
                continue;
            }

            /*
            #Find a malted flavor and switch it
            #If we find one, then since we never switch a malted back,
            #the customer should no longer be processed
            */
            if let Some(mc) = malted_prefs[c_idx] {
                assert!(unmalted.remove(mc), "should have been unmalted");
                choice_made = true;
                continue;
            } else {
                return None;
            }
        }
    }

    Some((0..num_flavors).map(|f| !unmalted.contains(f)).collect())
}
