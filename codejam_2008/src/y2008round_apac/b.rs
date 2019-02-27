use codejam::util::codejam::run_cases;
use codejam::util::grid::constants::*;
use codejam::util::grid::Grid;
use codejam::util::vector_2d::Vector2d;
//use itertools::max;
use std::cmp;
use std::collections::VecDeque;
use std::io::Write;

/*
Grid
Simulation
Brute force

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
                let my_loc = Vector2d::with_val(nums[3] - 1, nums[2] - 1);

                let mut grid = Grid::new(n_rows, n_cols);

                for r in 0..n_rows {
                    for (c, power) in reader.read_num_line().into_iter().enumerate() {
                        grid[(r, c)] = power;
                    }
                }

                if case_no != 4 {
                    //continue;
                }
                println!("Solving case {}", case_no);

                writeln!(
                    buffer,
                    "Case #{}: {}",
                    case_no,
                    if let Some(ans) = solve(my_loc.convert(), &grid) {
                        ans.to_string() + " day(s)"
                    } else {
                        "forever".to_string()
                    }
                )
                .unwrap();
            }
        },
    );
}

fn do_turn(
    me_location: Vector2d<isize>,
    attack_dir: Option<Vector2d<isize>>,
    grid: &mut Grid<isize>,
) -> bool
{
    let mut diffs: Grid<isize> = Grid::new(grid.R, grid.C);
    let mut ret = false;

    let mut cur_loc = Vector2d::with_val(0isize, 0);

    for r in 0..grid.R as isize {
        cur_loc.data[0] = r;
        for c in 0..grid.C as isize {
            cur_loc.data[1] = c;

            if cur_loc == me_location {
                if attack_dir.is_some() {
                    let adj_loc = cur_loc + attack_dir.unwrap();
                    if let Some(_attack_square) = grid.get_value(&adj_loc) {
                        diffs[&adj_loc] -= grid[&me_location];
                    }
                }
                continue;
            }

            if grid[&cur_loc] <= 0 {
                continue;
            }

            let strongest_neighbor = DIRECTIONS
                .iter()
                .enumerate()
                .filter_map(|(idx, dir)| {
                    let sq = cur_loc + dir;
                    if let Some(power) = grid.get_value(&sq) {
                        //-idx is to break ties by DIRECTIONS index
                        Some((*power, 5 - idx, sq))
                    } else {
                        None
                    }
                })
                .max()
                .unwrap();

            ret = ret || strongest_neighbor.0 > 0;

            diffs[&strongest_neighbor.2] -= grid[&cur_loc];
        }
    }

    //cout << "Diffs\n" << diffs;

    for r in 0..grid.R as isize {
        cur_loc.data[0] = r;
        for c in 0..grid.C as isize {
            cur_loc.data[1] = c;
            grid[&cur_loc] = cmp::max(0, grid[&cur_loc] + diffs[&cur_loc]);
        }
    }

    ret
}

const DIRECTIONS: [Vector2d<isize>; 4] = [NORTH, WEST, EAST, SOUTH];

fn solve(me_location: Vector2d<isize>, grid: &Grid<isize>) -> Option<isize>
{
    let mut q: VecDeque<(isize, Grid<isize>)> = VecDeque::new();
    q.push_back((0, grid.clone()));
    let mut max_t = 0;

    while let Some(item) = q.pop_front() {
        let item_grid: &Grid<isize> = &item.1;

        //we are dead
        if item_grid[&me_location] <= 0 {
            continue;
        }

        debug!(
            "Grid off stack: \n{:#.3?}\nTurns: {} my loc: {:?}",
            item.1, item.0, me_location
        );
        max_t = cmp::max(max_t, item.0);

        for attack_dir in DIRECTIONS.iter() {
            if let Some(attack_val) = item_grid.get_value(&(me_location + attack_dir)) {
                if *attack_val <= 0 {
                    continue;
                }
            } else {
                continue;
            }
            let mut new_grid: Grid<isize> = item_grid.clone();
            /*LOG_STR("Grid before: " << *newGrid
            << " turns: " << item.first
            << " Attacking: " << *adj_it);*/
            let did_move = do_turn(me_location, Some(*attack_dir), &mut new_grid);
            debug!(
                "After attacking in dir {:?} grid \n{:#.3?} Alive: {}",
                attack_dir, new_grid, item.0
            );
            q.push_back((item.0 + 1, new_grid));
            if !did_move {
                return None;
            }
        }

        let mut new_grid: Grid<isize> = item_grid.clone();

        //LOG_STR("Before doing nothing: " << *newGrid);
        let did_move = do_turn(me_location, None, &mut new_grid);
        debug!("After doing nothing: \n{:#.3?}\nAlive {}", new_grid, item.0);
        q.push_back((item.0 + 1, new_grid));
        //LOG_OFF();
        //LOG_STR();

        //LOG(item.first + 1);
        //return;
        if !did_move {
            return None;
        }
    }

    Some(max_t)
}
