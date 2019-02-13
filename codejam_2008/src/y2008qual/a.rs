//use bit_set::BitSet;
use codejam::util::codejam::run_cases;
use std::io::Write;
use std::usize;

/*
Simulation
Digits
*/
pub fn solve_all_cases()
{
    run_cases(
        &["A-small-practice", "A-large-practice"],
        "y2008qual",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let n = reader.read_int();

                let search_engines : Vec<_> = (0..n).map(|_| reader.read_string()).collect();

                let q = reader.read_int();

                let queries : Vec<_> = (0..q).map(|_| reader.read_string()).collect();

                if case_no != 3 {
                    // continue;
                }

                println!("Solving case {}", case_no);

                writeln!(
                    buffer,
                    "Case #{}: {}",
                    case_no,
                    solve(search_engines.as_slice(), queries.as_slice())
                )
                .unwrap();
            }
        },
    );
}

fn solve(search_engines: &[String], queries: &[String]) -> usize
{
    let mut s_changes = 0;
	let mut cur_q = 0;

	while cur_q != queries.len() {

        let s_potential_progress = search_engines.iter().map( |search| {
            let idx = queries[cur_q..queries.len()].iter().position( |q| q == search);
            if let Some(idx) = idx  {
                idx + cur_q
            } else {
                queries.len()
            }
        });

        cur_q = s_potential_progress.max().unwrap();
        s_changes += 1;
    }

	if s_changes > 0 {
        s_changes -= 1;
    }

    s_changes
}
