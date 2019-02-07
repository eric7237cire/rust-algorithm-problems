use codejam::util::codejam::run_cases;
//use std::cmp::max;
//use itertools::Itertools;
use std::io::Write;
//use std::mem::swap;

//use codejam::util::grid::constants::*;
use codejam::util::grid::{Grid, GridCoord, GridRowColVec, IntCoord2d};

//use permutohedron::LexicalPermutation;

/*

*/
pub fn solve_all_cases()
{
    run_cases(
        &[
            "C-small-practice",
            //"B-large-practice"
        ],
        "y2016round2",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let (R, C) = reader.read_tuple_2();

                let lovers = reader.read_num_line();

                assert_eq!(2 * (R + C), lovers.len());

                if case_no > 3 {
                    continue;
                }

                println!("Solving case {}", case_no);

                writeln!(
                    buffer,
                    "Case #{}: {}",
                    case_no,
                    solve(R, C, &lovers) //solve_brute_force(K, &prob)
                )
                .unwrap();
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

    let mut matches = vec![ 0; 2* (R+C) + 1];
    for matched in lovers.windows(2) {
        let L1 = matched[0];
        let L2 = matched[1];
        matches[L1] = L2;
        matches[L2] = L1;
    }

    let grid_coords_to_lover = |r: usize, c: usize| {
        //top
        if r == 0 {
            return Some(c);
        }
        //right
        if c == C + 1 {
            return Some(r + C);
        }
        //bottom
        if r == 1 + R {
            return Some(2 * C + R - c + 1);
        }
        //left
        if c == 0 {
            return Some(2 * (R + C) - r + 1);
        }

        None
    };

    for subset in 0..1 << (R * C) {
        let mut grid: Grid<String> = Grid::new(R + 2, C + 2);

        //top
        for label in 0..C {
            grid[(0, 1 + label)] = (label + 1).to_string();
            assert_eq!(Some(label+1), grid_coords_to_lover(0, 1+label));
        }
        //right
        for label in C..C + R {
            grid[(1 + label - C, C + 1)] = (label + 1).to_string();
            assert_eq!(Some(label+1), grid_coords_to_lover(1 + label - C, C + 1));
        }
        //bottom
        for label in C + R..2 * C + R {
            grid[(1 + R, 2 * C + R - label)] = (label + 1).to_string();
            assert_eq!(Some(label+1), grid_coords_to_lover(1 + R, 2 * C + R - label));
        }
        //left
        for label in 2 * C + R..2 * (R + C) {
            grid[(2 * (R + C) - label, 0)] = (label + 1).to_string();
            assert_eq!(Some(label+1), grid_coords_to_lover(2 * (R + C) - label, 0));
        }

        for row in 0..R {
            for col in 0..C {
                let index = row * C + col;
                let is_forward = (subset >> index) & 1 > 0;
                grid[(row + 1, col + 1)] = if is_forward {
                    "/".to_string()
                } else {
                    "\\".to_string()
                };
            }
        }

        debug!("Subset {:b} Grid\n{:#.4?}\n", subset, grid);
    }

    "IMPOSSIBLE".to_string()
}
