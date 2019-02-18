//use bit_set::BitSet;
use codejam::util::bitvec64::BitVec64;
use codejam::util::codejam::run_cases;
use codejam::util::grid::constants::*;
use codejam::util::vector_2d::Vector2d;
use itertools::Itertools;
use std::char::from_digit;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::i64;
use std::io::Write;

/*
Grid
Maze finding
*/
pub fn solve_all_cases()
{
    run_cases(
        &["B-small-practice", "B-large-practice"],
        "y2008practice",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let in_s = reader.read_string_line();

                if case_no != 3 {
                    // continue;
                }

                println!("Solving case {}", case_no);

                write!(buffer, "Case #{}:\n{}", case_no, solve(&in_s[0], &in_s[1])).unwrap();
            }
        },
    );
}

fn solve(forward_path: &str, back_path: &str) -> String
{
    let zeropos = Vector2d::with_val(0, 0);

    let directions = [NORTH, SOUTH, WEST, EAST];

    let get_dir_index = |dir: &Vector2d<i64>| directions.iter().position(|dd| dd == dir).unwrap();

    let mut square_state: HashMap<Vector2d<i64>, BitVec64> = HashMap::new();

    let mut handle_step = |step: char, dir: &mut Vector2d<i64>, pos: &mut Vector2d<i64>| {
        debug!("Read a step {}", step);
        match step {
            'W' => {
                square_state
                    .entry(*pos)
                    .or_insert_with(|| BitVec64::new())
                    .set(get_dir_index(dir), true);
                *pos += *dir;
                square_state
                    .entry(*pos)
                    .or_insert_with(|| BitVec64::new())
                    .set(get_dir_index(&dir.rotate_rc_reverse()), true);
            }
            'R' => {
                dir.rotate_rc_right_mut();
            }
            'L' => {
                dir.rotate_rc_left_mut();
            }
            _ => panic!("Invalid char"),
        }

        debug!("Current pos {:?}, direction {:?}", pos, dir);
    };

    let mut handle_path =
        |path: &str, initial_dir: Vector2d<i64>, initial_pos: Vector2d<i64>| {
            let mut dir = initial_dir;
            let mut pos = initial_pos;
            for ch in path.chars() {
                handle_step(ch, &mut dir, &mut pos);
            }
            (dir, pos)
        };

    let (dir, pos) = handle_path(forward_path, SOUTH, Vector2d::with_val(0, 0));
    let mut dir = dir;
    dir.rotate_rc_reverse_mut();

    let exit_pos = pos;

    debug!("Handle back bath starting at {:?}", pos);
    handle_path(back_path, dir, pos);

    square_state.remove(&zeropos);
    square_state.remove(&exit_pos);

    let mut min_row = i64::MAX;
    let mut max_row = i64::MIN;
    let mut min_col = i64::MAX;
    let mut max_col = i64::MIN;
    for pos in square_state.keys() {
        min_row = min(pos.r(), min_row);
        max_row = max(pos.r(), max_row);
        min_col = min(pos.c(), min_col);
        max_col = max(pos.c(), max_col);
    }

    let mut ans = Vec::new();
    for r in min_row..=max_row {
        for c in min_col..=max_col {
            debug!("R: {} C: {}", r, c);
            ans.push(from_digit(square_state[&Vector2d::with_val(r, c)].data as u32, 16).unwrap());
        }
        ans.push('\n');
    }

    ans.iter().join("")
}
