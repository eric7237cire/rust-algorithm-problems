use std::io::Write;
use codejam::util::codejam::run_cases;

/*
grid
handle constraints

TODO use grid class
*/
pub fn solve_all_cases()
{
    run_cases(
        &["A-small-practice", "A-large-practice"],
        "y2017round1a",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {

                //handle input / output
                let (r, _c) = reader.read_tuple_2();

                let mut grid: Vec<Vec<char>> = Vec::new();

                for _ in 0..r {
                    let s = reader.read_string();
                    grid.push(s.chars().collect());
                }


                write!(buffer, "Case #{}:{}", case_no, solve( &mut grid)).unwrap();
            }
        });

}

fn handle_col(r: usize, c: usize, grid: &mut Vec<Vec<char>>, last_value: &mut char)
{
    let value = grid[r][c];

    if value != '?' {
        *last_value = value;
        return;
    }

    if *last_value != '?' && value == '?' {
        grid[r][c] = *last_value;
    }
}

fn solve( grid: &mut Vec<Vec<char>>) -> String
{
    //let mut last_value: char = '?';

    let n_rows = grid.len();
    let n_cols = grid[0].len();

    for c in 0..n_cols {
        let mut last_value = '?';
        for r in 0..n_rows {
            handle_col(r, c, grid, &mut last_value);
        }

        last_value = '?';
        for r in (0..n_rows).rev() {
            handle_col(r, c, grid, &mut last_value);
        }
    }

    // Now handle blank columns

    // Copy to right
    for c in 1..n_cols {
        if grid[0][c] == '?' {
            for r in 0..n_rows {
                grid[r][c] = grid[r][c - 1];
            }
        }
    }

    // Copy to left (from right)
    for c in (0..n_cols - 1).rev() {
        if grid[0][c] == '?' {
            for r in 0..n_rows {
                grid[r][c] = grid[r][c + 1];
            }
        }
    }

    let mut ans = "\n".to_string();
    for r in 0..n_rows {
        for c in 0..n_cols {
            ans += &grid[r][c].to_string();
        }
        ans += "\n"
    }
    ans
}
