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

                writeln!(buffer, "Case #{}: {}", case_no, 
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
       let mut grid: Grid<String> = Grid::new(R+2, C+2);

        //top
       for label in 0..C {
           grid[ (0, 1+label) ] = (label+1).to_string();
       }
       //right
       for label in C..C+R {
           grid[ (1+label-C, C+1) ] = (label+1).to_string();
       }
       //bottom
       for label in C+R..2*C+R {
           grid[ (1+R, 2*C+R-label) ] = (label+1).to_string();
       }
       //left
       for label in 2*C+R..2*(R+C) {
           grid[ (2*(R+C)-label, 0) ] = (label+1).to_string();
       }

       for row in 0..R {
           for col in 0..C {
               let index = row * C + col;
                let is_forward = (subset >> index) & 1 > 0;
                grid[ (row+1, col+1) ] = if is_forward { "/".to_string() } else { "\\".to_string() };
           }
       }

       debug!("Subset {:b} Grid\n{:#.4?}\n", subset, grid);
   }
   
    

    "IMPOSSIBLE".to_string()
}