use codejam::util::codejam::run_cases;
//use std::cmp::max;
//use itertools::Itertools;
use std::io::Write;
//use std::mem::swap;

//use codejam::util::grid::constants::*;
use codejam::util::grid::constants::EAST;
use codejam::util::grid::constants::NORTH;
use codejam::util::grid::constants::SOUTH;
use codejam::util::grid::constants::WEST;
use codejam::util::grid::{Grid, GridRowColVec, IntCoord2d};
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;

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

                if case_no > 5 {
                    //continue;
                }

                println!("Solving case {}", case_no);

                writeln!(
                    buffer,
                    "Case #{}:\n{}",
                    case_no,
                    solve(R, C, &lovers) //solve_brute_force(K, &prob)
                )
                .unwrap();
            }
        },
    );
}

fn grid_coords_to_lover(r: usize, c: usize, R: usize, C: usize) -> Option<usize>
{
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
}

fn next_dir_loc(loc: &mut IntCoord2d<usize>, dir: IntCoord2d<i64>, grid: &Grid<String>) -> IntCoord2d<i64>
{
   let new_dir = match dir {
        NORTH => {
            if grid[*loc] == "/" {
                EAST
            } else {
                WEST
            }
        }
        SOUTH => {
            if grid[*loc] == "/" {
                WEST
            } else {
                EAST
            }
        }
        EAST => {
            if grid[*loc] == "/" {
                NORTH
            } else {
                SOUTH
            }
        }
        WEST => {
            if grid[*loc] == "/" {
                SOUTH
            } else {
                NORTH
            }
        },
        _ => panic!("odd direction")
    };

    *loc += new_dir;

    new_dir

}

fn trace_path(initial_loc: IntCoord2d<usize>, grid: &Grid<String>, initial_direction: GridRowColVec) -> usize
{
    assert!(grid[initial_loc] == "/" || grid[initial_loc] == "\\");

    //Without lover ring
    let R = grid.R - 2;
    let C = grid.C - 2;

    let mut loc = initial_loc;
    let mut dir = initial_direction;

    loop {

        dir = next_dir_loc(&mut loc, dir, grid);

        if let Some(lover) = grid_coords_to_lover(loc.0, loc.1, R, C) {
            return lover;
        }
    }
}

fn solve_brute_force(R: usize, C: usize, lovers: &[usize]) -> String
{
    //need 2 * R * C nodes
    //top is even, bottom is odd

    //Go through every subset
    assert!(R * C <= 16);

    let mut matches = vec![0; 2 * (R + C) + 1];
    for matched in lovers.chunks_exact(2) {
        let L1 = matched[0];
        let L2 = matched[1];
        matches[L1] = L2;
        matches[L2] = L1;
    }

    debug!("Matches: {:?}", matches);

    for subset in 0..1 << (R * C) {
        let mut grid: Grid<String> = Grid::new(R + 2, C + 2);
        let mut grid_garden : Grid<char> = Grid::new(R,C);

        for row in 0..R {
            for col in 0..C {
                let index = row * C + col;
                let is_forward = (subset >> index) & 1 > 0;
                grid[(row + 1, col + 1)] = if is_forward {
                    "/".to_string()
                } else {
                    "\\".to_string()
                };

                grid_garden[(row, col)] = if is_forward {
                    '/'
                } else {
                    '\\'
                };
            }
        }

        let mut lover_locations = Vec::new();

        let mut add_lover = |r,c, label:usize, initial_dir| {
            grid[(r, c)] = (label + 1).to_string();
            assert_eq!(Some(label + 1), grid_coords_to_lover(r, c, R, C));

            lover_locations.push(((label + 1), IntCoord2d(r, c), initial_dir));
        };

        //top
        for label in 0..C {
            let r = 0;
            let c = 1 + label;

            add_lover(r,c,label, SOUTH);
        }
        //right
        for label in C..C + R {
            let r = 1 + label - C;
            let c = C + 1;
            add_lover(r,c,label, WEST);
        }
        //bottom
        for label in C + R..2 * C + R {
            let r = 1 + R;
            let c = 2 * C + R - label;
            add_lover(r,c,label, NORTH);
        }
        //left
        for label in 2 * C + R..2 * (R + C) {
            let r = 2 * (R + C) - label;
            let c = 0;
            add_lover(r,c,label, EAST);
        }

        debug!("Subset {:b} Grid\n{:#.3?}\n", subset, grid);

        let mut valid = true;

        //top
        for (lover, loc, dir) in lover_locations {
            let other_lover = trace_path(loc + dir, &grid, dir);
            if matches[lover] != other_lover {
                valid = false;
                debug!("Lover {} in location {:?} matched with {}.  Valid? {}",
lover, loc, other_lover, valid);
                //break;
            }
        }

        if valid {
            return format!("{}", Gwrapper {grid: grid_garden});
        }
    }

    "IMPOSSIBLE".to_string()
}

struct Lover
{
    number: usize,
    location: IntCoord2d<i64>,
    initial_direction: IntCoord2d<i64>,
}

struct LoverPair
{
    L1: Lover,
    L2: Lover,
    distance: i64
}

fn solve(R: usize, C: usize, lover_pairings: &[usize]) -> String
{
    //need 2 * R * C nodes
    //top is even, bottom is odd

    //Go through every subset
    assert!(R * C <= 16);



    let mut grid: Grid<String> = Grid::new(R + 2, C + 2);

    let mut grid_garden : Grid<char> = Grid::new(R,C);

    let mut lovers: Vec<Lover> = Vec::new();

    let mut add_lover = |r,c, label:usize, initial_dir| {
        grid[(r, c)] = (label + 1).to_string();
        assert_eq!(Some(label + 1), grid_coords_to_lover(r, c, R, C));

        lovers.push(Lover{ number: label + 1,
            location:IntCoord2d(r as i64, c as i64), initial_direction:initial_dir});
    };

    //top
    for label in 0..C {
        let r = 0;
        let c = 1 + label;

        add_lover(r,c,label, SOUTH);
    }
    //right
    for label in C..C + R {
        let r = 1 + label - C;
        let c = C + 1;
        add_lover(r,c,label, WEST);
    }
    //bottom
    for label in C + R..2 * C + R {
        let r = 1 + R;
        let c = 2 * C + R - label;
        add_lover(r,c,label, NORTH);
    }
    //left
    for label in 2 * C + R..2 * (R + C) {
        let r = 2 * (R + C) - label;
        let c = 0;
        add_lover(r,c,label, EAST);
    }

    debug!("Grid\n{:#.3?}\n", grid);

    let mut matches:Vec<LoverPair> = Vec::new();
    for matched in lover_pairings.chunks_exact(2) {
        let L1_num = matched[0];
        let L2_num = matched[1];
        let L1 = lovers.iter().find(|lov| lov.number == L1_num).unwrap();
        let L2 = lovers.iter().find(|lov| lov.number == L2_num).unwrap();

        matches.push( LoverPair{ L1:*L1, L2:*L2, distance: (L1.location-L2.location).abs() );
    }

    debug!("Matches: {:?}", matches);


    "IMPOSSIBLE".to_string()
}


struct Gwrapper
{
    grid: Grid<char>
}


impl Display for Gwrapper
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        for r in 0..self.grid.R {
            for c in 0..self.grid.C {
                if let Err(err) = write!(f, "{}", self.grid[(r, c)]) {
                    return Err(err);
                }
            }
            if let Err(err) = writeln!(f, "") {
                return Err(err);
            }
        }
        write!(f, "")
    }
}