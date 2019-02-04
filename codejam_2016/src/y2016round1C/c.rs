use codejam::util::codejam::run_cases;

use itertools::Itertools;
use std::io::Write;

use std::collections::HashMap;

/*

*/
pub fn solve_all_cases()
{
    run_cases(
        &[
            "C-small-practice",
            //"B-large-practice"
        ],
        "y2016round1C",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let nums = reader.read_num_line();

                if case_no != 1 {
                    //continue;
                }

                println!("Solving case {}", case_no);

                writeln!(buffer, "Case #{}: {}", case_no, solve(nums[0], nums[1], nums[2], nums[3])).unwrap();
            }
        },
    );
}


fn solve(J: usize, P: usize, S: usize, K: usize) -> String
{

    let mut perms: Vec< [usize;3] > = Vec::new();
    for j in 1..=J {
        for p in 1..=P {
            for s in 1..=S {
                perms.push( [j,p,s] );
            }
        }
    }

    if perms.len() > 18 {
        //return "Too long".to_string();
    }

    assert!(perms.len() <= 27);

    let mut constraint_count : HashMap< [usize;2], usize> = HashMap::new();

    let mut best_count = 0;
    let mut best_ans = String::new();

    //Now iterate over every subset
    'perms_loop: for subset in 0..1<<perms.len() 
    {
        let mut count = 0;
        for (p_idx,p) in perms.iter().enumerate()
        {
            if subset >> p_idx & 1 ==0 {
                continue;
            }
            count += 1;

            let cons_count = constraint_count.entry( [p[0], p[1]]).or_insert(0);
            *cons_count += 1;

            if *cons_count > K {
                continue 'perms_loop;
            }

        }

        if count > best_count {
            best_count = count;
            best_ans = format!("{}\n{}",
            best_count,
            perms.iter().map( |p| p.iter().join(" ") ).join("\n"));
        }
    }


    best_ans

}
