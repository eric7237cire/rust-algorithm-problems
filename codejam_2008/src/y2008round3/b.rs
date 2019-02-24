use codejam::util::codejam::run_cases;
use codejam::util::grid::constants::*;
use codejam::util::vector_2d::Vector2d;
use std::io::Write;
use codejam::util::grid::Grid;
use std::usize;
use std::cmp::min;

/*
Polygons
*/
pub fn solve_all_cases()
{
    run_cases(
        &["B-small-practice",
            "B-large-practice"
        ],
        "y2008round3",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let (n_rows,n_cols) = reader.read_tuple_2();

                let mut grid = Grid::new(n_rows,n_cols);

                for r in 0..n_rows {
                    for (c, ch) in reader.read_chars(n_cols).into_iter().enumerate() {
                        grid[ (r,c) ] = ch;
                    }
                }

                if case_no != 1 {
                    continue;
                }
                println!("Solving case {}", case_no);

                writeln!(buffer, "Case #{}: {}", case_no, if let Some(ans) = solve(&grid) { ans.to_string() } else {
                    "THE CAKE IS A LIE".to_string()
                }).unwrap();
            }
        },
    );
}

/*
. indicates an empty cell;
# indicates a wall;
O indicates your starting position; and
X indicates the cake's position.
*/

const DIRECTIONS: [Vector2d<isize>; 4] = [NORTH, EAST, SOUTH, WEST];

fn solve(grid: &Grid<char>) -> Option<u32>
{
    debug!("Grid\n{:#.4?}\n", grid);

    //precompute 5 values, closest wall in 4 directions, then closest wall absolutely
    let mut nearest_dir: Grid< [usize; 5] > = Grid::new(grid.R, grid.C);

    for c in nearest_dir.data.iter_mut() {
        c[4] = usize::MAX;
    }

    for r in 0..grid.R  {
        for c in (0..grid.C).rev()  {
            let coord = Vector2d::with_val(r as isize,c as isize);
            for (dir_idx, dir) in [NORTH, EAST].iter().enumerate() {

                for &dd_idx in [dir_idx, 4].iter() {
                    nearest_dir.data[r * grid.C + c][dd_idx] = if grid[&coord] == '#' {
                        0
                    } else if let Some(d) = nearest_dir.get_value(&(coord + dir)) {
                        1 + d[dd_idx]
                    } else {
                        1
                    };
                }

            }


        }
    }

    for r in (0..grid.R).rev()  {
        for c in 0..grid.C  {
            let coord = Vector2d::with_val(r as isize,c as isize);
            for (dir_idx, dir) in [SOUTH, WEST].iter().enumerate() {

                for &dd_idx in [dir_idx, 4].iter() {
                    nearest_dir.data[r * grid.C + c][dd_idx] = if grid[&coord] == '#' {
                        0
                    } else if let Some(d) = nearest_dir.get_value(&(coord + dir)) {
                        1 + d[dd_idx]
                    } else {
                        1
                    };
                }

            }


        }
    }

    for r in 0..grid.R {
        for c in 0..grid.C {
            debug!("For r {} c {} --> NESW near {:?}", r, c, nearest_dir[(r, c)]);
        }
    }

    Some(3)
}