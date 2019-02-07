use codejam::util::codejam::run_cases;
//use std::cmp::max;
//use itertools::Itertools;
use std::io::Write;
use std::mem::swap;

use codejam::util::grid::constants::*;
use codejam::util::grid::{Grid, GridCoord, GridRowColVec, IntCoord2d};

//use permutohedron::LexicalPermutation;

/*

*/
pub fn solve_all_cases()
{
    run_cases(
        &["C-small-practice",
         //"B-large-practice"
        ],
        "y2016round2",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                

                let (R,C) = reader.read_tuple_2();
                
                let lovers = reader.read_num_line();

                assert_eq!(2 * (R+C), lovers.len());

                if case_no > 3 {
                     continue;
                }

                println!("Solving case {}", case_no);

                writeln!(buffer, "Case #{}: {:.8}", case_no, 
                solve(R, C, &lovers)
                //solve_brute_force(K, &prob)
                ).unwrap();
            }
        },
    );
}


fn solve(R: usize, C: usize, lovers: &[usize]) -> String
{
   //need 2 * R * C nodes
   //top is even, bottom is odd

   //Go through every subset
    assert!(R * C <= 16);

   for subset in 0..1 << (R*C) {
       let mut g: Grid<char> = Grid::new(R, C);

       for index in 0..R*C {
           let is_forward = (subset >> index) & 1 > 0;
           g[index] = if is_forward { '/' } else { '\\' };
       }

       debug!("Subset {:b} Grid\n{:#.4?}\n", subset, g);
   }
   
    

    "IMPOSSIBLE".to_string()
}