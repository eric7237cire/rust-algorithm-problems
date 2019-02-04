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

                if case_no != 2 {
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

    println!("Starting J={} P={} S={}  K={}", J,P,S,K);

    

    let mut best_count = 0;
    let mut best_ans = String::new();

    //Now iterate over every subset
    'perms_loop: for subset in 0..1<<perms.len() 
    {
        let mut count = 0;
        let mut constraint_jp_count : HashMap< [usize;2], usize> = HashMap::new();
        let mut constraint_js_count : HashMap< [usize;2], usize> = HashMap::new();
        let mut constraint_ps_count : HashMap< [usize;2], usize> = HashMap::new();

        debug!("Starting\n{}",
        perms.iter().enumerate().filter( |(p_idx,_)|
            subset >> p_idx & 1 > 0).map(| (_, p) | p.iter().join(" ")).join("\n"));

        for (p_idx,p) in perms.iter().enumerate()
        {
            if subset >> p_idx & 1 == 0 {
                continue;
            }
            count += 1;

            let cons_count = constraint_jp_count.entry( [p[0], p[1]]).or_insert(0);
            *cons_count += 1;

            if *cons_count > K {
            /*    debug!("Constraint count of [{},{}] {} > K {}",
                p[0], p[1], cons_count, K );*/
                continue 'perms_loop;
            }

            let cons_count = constraint_js_count.entry( [p[0], p[2]]).or_insert(0);
            *cons_count += 1;

            if *cons_count > K {
            /*    debug!("Constraint count of [{},{}] {} > K {}",
                p[0], p[1], cons_count, K );*/
                continue 'perms_loop;
            }

            let cons_count = constraint_ps_count.entry( [p[1], p[2]]).or_insert(0);
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
            best_ans = format!("{}\n{}",
            best_count,
            perms.iter().enumerate().filter( |(p_idx,_)|
            subset >> p_idx & 1 > 0).map(| (_, p) | p.iter().join(" ")).join("\n"));
        }
    }


    best_ans

}
