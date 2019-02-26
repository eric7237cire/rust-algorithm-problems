use bit_set::BitSet;
use codejam::util::codejam::run_cases;
use codejam::util::grid::constants::*;
use codejam::util::grid::Grid;
use codejam::util::vector_2d::Vector2d;
use std::cmp::min;
use std::collections::BinaryHeap;
use std::io::Write;
use std::usize;

/*
Grid
Dijkstras / priority queue (using negative to make it a min queue)

*/
pub fn solve_all_cases()
{
    run_cases(
        &["B-small-practice", "B-large-practice"],
        "y2008round_apac",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let nums = reader.read_num_line();
                let n_cols = nums[0];
                let n_rows = nums[1];
                let my_loc = Vector2d::with_val(nums[2], nums[3]);

                let mut grid = Grid::new(n_rows, n_cols);


                for r in 0..n_rows {
                    for (c, power) in reader.read_num_line().into_iter().enumerate() {
                        grid[(r, c)] = power;
                    }
                }

                if case_no != 1 {
                    // continue;
                }
                println!("Solving case {}", case_no);

                writeln!(
                    buffer,
                    "Case #{}: {}",
                    case_no,
                    if let Some(ans) = solve(&grid, &start, &stop) {
                        ans.to_string()
                    } else {
                        "THE CAKE IS A LIE".to_string()
                    }
                )
                .unwrap();
            }
        },
    );
}

const DIRECTIONS: [Vector2d<isize>; 4] = [NORTH, EAST, SOUTH, WEST];

const CLOSEST_WALL_INDEX: usize = 4;

fn solve(grid: &Grid<char>, start: &Vector2d<isize>, stop: &Vector2d<isize>) -> Option<isize>
{
    debug!("Grid\n{:#.4?}\n", grid);

    //precompute 5 values, closest wall in 4 directions, then closest wall absolutely
    let mut nearest_dir: Grid<[usize; 5]> = Grid::new(grid.R, grid.C);

    for c in nearest_dir.data.iter_mut() {
        c[4] = usize::MAX;
    }

    for rr in 0..grid.R {
        for cc in 0..grid.C {
            //start from top/left corner or bottom/right corner.
            for &(r, c, dir_start) in [(rr, grid.C - 1 - cc, 0), (grid.R - 1 - rr, cc, 2)].iter() {
                let coord = Vector2d::with_val(r as isize, c as isize);
                for (dir_idx, dir) in DIRECTIONS.iter().enumerate().skip(dir_start).take(2) {
                    for &dd_idx in [dir_idx, CLOSEST_WALL_INDEX].iter() {
                        let val = if grid[&coord] == '#' {
                            0
                        } else if let Some(d) = nearest_dir.get_value(&(coord + dir)) {
                            1 + d[dd_idx]
                        } else {
                            1
                        };
                        let m_val = nearest_dir.data[r * grid.C + c].get_mut(dd_idx).unwrap();
                        if dd_idx == CLOSEST_WALL_INDEX {
                            *m_val = min(*m_val, val);
                        } else {
                            *m_val = val;
                        }
                    }
                }
            }
        }
    }

    for r in 0..grid.R {
        for c in 0..grid.C {
            debug!(
                "For r {} c {} --> NESW near {:?}",
                r,
                c,
                nearest_dir[(r, c)]
            );
        }
    }

    let mut pq: BinaryHeap<(isize, Vector2d<isize>)> = BinaryHeap::new();
    let mut visited = BitSet::new();
    pq.push((0, start.clone()));

    while let Some(node) = pq.pop() {
        let cur = node.1;
        let idx = cur.r() as usize * grid.C + cur.c() as usize;
        debug!("Cur is {:?} idx is {}", cur, idx);

        let cost = node.0;
        if visited.contains(idx) {
            continue;
        }
        if &cur == stop {
            return Some(-cost);
        }
        visited.insert(idx);

        for (dir_idx, dir) in DIRECTIONS.iter().enumerate() {
            let d = nearest_dir[idx][dir_idx];
            //either we walk
            if d > 0 && grid.get_value(&(cur + dir)).is_some() {
                debug!(
                    "Walking Cur is {:?} adding {:?} for dir {}",
                    cur,
                    cur + dir,
                    dir_idx
                );

                pq.push((cost - 1, cur + dir));
            }
            //or shoot a portal and go to the closest wall to teleport to it
            if d > 1 {
                debug!(
                    "Cur is {:?} adding {:?} for dir {}",
                    cur,
                    cur + &(dir * (d - 1) as isize),
                    dir_idx
                );
                pq.push((
                    cost - nearest_dir[idx][CLOSEST_WALL_INDEX] as isize,
                    cur + &(dir * (d - 1) as isize),
                ));
            }
        }
    }

    None
}
