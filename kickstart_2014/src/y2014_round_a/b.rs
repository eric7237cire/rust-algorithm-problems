//use bit_set::BitSet;
use codejam::util::codejam::run_cases;
use std::cmp::min;
use std::cmp::Ordering;
use std::io::Write;
use std::usize;
use codejam::util::grid::Grid;

/*
Grid
*/
const DIR_STRINGS : [&str; 4] = ["left", "right", "up", "down"];

pub fn solve_all_cases()
{
    run_cases(
        &["B-small-practice", "B-large-practice"],
        "y2014_round_a",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let in_line = reader.read_string_line();
                assert_eq!(2, in_line.len());
                let n : usize = in_line[0].parse().unwrap();
                let dir = in_line[1];

                let mut grid = Grid::new(n,n);

                for r in 0..n {
                    let grid_row = reader.read_num_line();
                    assert_eq!(n, grid_row.len());
                    for (c, val) in grid_row.into_iter() {
                        grid[(r, c)] = val;
                    }
                }

                if case_no != 1 {
                    // continue;
                }

                println!("Solving case {}", case_no);

                let ans = solve(t, arrivals.as_slice(), departures.as_slice());
                writeln!(buffer, "Case #{}: {} {}", case_no, ans[0], ans[1]).unwrap();
            }
        },
    );
}

fn solve(dir: String, grid: &Grid<u16>) -> String
{


    "3".to_string()
}
