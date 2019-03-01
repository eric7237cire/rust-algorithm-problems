//use bit_set::BitSet;
use codejam::util::codejam::run_cases;
use std::io::Write;
use std::usize;
use codejam::util::grid::Grid;
use codejam::util::vector_2d::Vector2d;
use codejam::util::grid::constants::*;
use itertools::Itertools;

/*
Grid
*/

pub fn solve_all_cases()
{
    run_cases(
        &[
            "B-sample",
            "B-small-practice",
            "B-large-practice"
        ],
        "y2014_round_a",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let in_line = reader.read_string_line();
                assert_eq!(2, in_line.len());
                let n : usize = in_line[0].parse().unwrap();
                let dir = &in_line[1];

                let mut grid = Grid::new(n,n);

                for r in 0..n {
                    let grid_row = reader.read_num_line();
                    assert_eq!(n, grid_row.len());
                    for (c, val) in grid_row.into_iter().enumerate() {
                        grid[(r, c)] = val;
                    }
                }

                if case_no != 1 {
                    // continue;
                }

                println!("Solving case {}", case_no);

                let ans = solve(&dir, &mut grid);
                writeln!(buffer, "Case #{}:\n{}", case_no, ans).unwrap();
            }
        },
    );
}

fn move_to_empty(grid: &mut Grid<u16>, start_square: &Vector2d<isize>, dir: &Vector2d<isize>) -> bool
{
    let n = grid.R as isize;
    let mut moved_any = false;
    let mut i = 0;
    while i < n - 1 {
        let cur_loc = *start_square + (dir * i);
        let next_loc = *start_square + (dir * (i + 1));

        if grid[&cur_loc] != 0 && grid[&next_loc] == 0 {
            grid[&next_loc] = grid[&cur_loc];
            grid[&cur_loc] = 0;
            moved_any = true;
        }
        i += 1;
    }

    moved_any
}

fn solve(dir: &str, grid: &mut Grid<u16>) -> String
{
    let n = grid.R as isize;
    let (start_range, step): (Vec<Vector2d<isize>>, Vector2d<isize>) = match dir {
        "left" => {
            ((0..n).map(|r| Vector2d::with_val(r, n - 1)).collect(),
             WEST)
        },
        "right" => {
            ((0..n).map(|r| Vector2d::with_val(r, 0)).collect(),
             EAST
            )
        },
        "up" => {
            ((0..n).map(|c| Vector2d::with_val(n-1, c)).collect(),
             NORTH
            )
        },
        "down" => {
            ((0..n).map(|c| Vector2d::with_val(0, c)).collect(),
             SOUTH
            )
        },
        _ => {
            panic!("Unrecognized str");
        }
    };

    //combining run
    for start_square in start_range.iter() {

        while move_to_empty(grid, start_square, &step) {}

        let mut i = n-1;
        while i > 0 {
            let cur_loc = *start_square + (step * i);
            let next_loc = *start_square + (step * (i-1));
            if grid[ & cur_loc ] == grid[ &next_loc ] {
                grid[ &next_loc] = 0;
                grid[ &cur_loc] *= 2;
                //make sure we don't recombine
                i -= 1;
            }
            i -= 1;
        }

        while move_to_empty(grid, start_square, &step) {}

    }

    (0..n).map( |r| (0..n).map( |c| grid[ (r,c) ]).join(" ") ).join("\n")
}
