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
use codejam::util::grid::{Grid};
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;
use codejam::util::vector_2d::Vector2d;
use bit_vec::BitVec;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

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

                if case_no != 8 {
                    continue;
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


fn next_dir_loc_2(loc: &GardenLocation, is_forward_slash: bool) -> GardenLocation
{

   let new_dir = match loc.entry_dir
   {
        NORTH => {
            if is_forward_slash {
                EAST
            } else {
                WEST
            }
        },
        SOUTH => {
            if is_forward_slash {
                WEST
            } else {
                EAST
            }
        },
        EAST => {
            if is_forward_slash {
                NORTH
            } else {
                SOUTH
            }
        },
        WEST => {
            if is_forward_slash {
                SOUTH
            } else {
                NORTH
            }
        },
        _ => panic!("odd direction")
    };

    GardenLocation{ grid_loc: loc.grid_loc + &new_dir,
    entry_dir: new_dir }

}

fn next_dir_loc(loc: &mut Vector2d<i64>, dir: &Vector2d<i64>, grid: &Grid<String>) -> Vector2d<i64>
{
    let loc_loc = &*loc;

   let new_dir = match dir {
        &NORTH => {
            if grid[loc_loc] == "/" {
                EAST
            } else {
                WEST
            }
        }
        &SOUTH => {
            if grid[loc_loc] == "/" {
                WEST
            } else {
                EAST
            }
        }
        &EAST => {
            if grid[loc_loc] == "/" {
                NORTH
            } else {
                SOUTH
            }
        }
        &WEST => {
            if grid[loc_loc] == "/" {
                SOUTH
            } else {
                NORTH
            }
        },
        _ => panic!("odd direction")
    };

    *loc += &new_dir;

    new_dir

}

#[allow(dead_code)]
fn trace_path(initial_loc: &Vector2d<i64>, grid: &Grid<String>, initial_direction: &Vector2d<i64>) -> usize
{
    assert!(grid[initial_loc] == "/" || grid[initial_loc] == "\\");

    //Without lover ring
    let R = grid.R - 2;
    let C = grid.C - 2;

    let mut loc : Vector2d<i64> = initial_loc.clone();
    let mut dir : Vector2d<i64> = initial_direction.clone();

    loop {

        dir = next_dir_loc(&mut loc, &dir, grid);

        if let Some(lover) = grid_coords_to_lover(loc.data[0] as usize, loc.data[1] as usize, R, C) {
            return lover;
        }
    }
}

#[allow(dead_code)]
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

            lover_locations.push(((label + 1), Vector2d::with_val(r as i64, c as i64), initial_dir));
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
            let other_lover = trace_path(&(loc.clone() + &dir), &grid, &dir);
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

#[derive(Debug, Clone)]
struct Lover
{
    number: usize,
    location: Vector2d<i64>,
    initial_direction: &'static Vector2d<i64>,
}

#[derive(Debug)]
struct LoverPair
{
    L1: Lover,
    L2: Lover,
    distance: i64
}

#[derive(Ord, PartialOrd, PartialEq, Eq, Hash, Copy, Clone)]
struct GardenLocation
{
    grid_loc: Vector2d<i64>,
    entry_dir: Vector2d<i64>
}

#[derive(Eq, PartialEq, Copy, Clone)]
struct HeapNode
{
    distance_to_target: i64,
    loc: GardenLocation
}

impl Ord for HeapNode
{
    fn cmp(&self, other: &HeapNode) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.distance_to_target.cmp(&self.distance_to_target)
            .then_with(|| self.loc.cmp(&other.loc)
            )
    }
}
impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &HeapNode) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve(R: usize, C: usize, lover_pairings: &[usize]) -> String
{
    //need 2 * R * C nodes
    //top is even, bottom is odd

    //Go through every subset
    assert!(R * C <= 16);



    let mut grid: Grid<String> = Grid::new(R + 2, C + 2);

    // let mut grid_garden : Grid<char> = Grid::new(R,C);

    let mut lovers: Vec<Lover> = Vec::new();

    let mut add_lover = |r,c, label:usize, initial_dir| {
        grid[(r, c)] = (label + 1).to_string();
        assert_eq!(Some(label + 1), grid_coords_to_lover(r, c, R, C));

        lovers.push(Lover{ number: label + 1,
            location:Vector2d::with_val(r as i64, c as i64), initial_direction:initial_dir});
    };

    //top
    for label in 0..C {
        let r = 0;
        let c = 1 + label;

        add_lover(r,c,label, &SOUTH);
    }
    //right
    for label in C..C + R {
        let r = 1 + label - C;
        let c = C + 1;
        add_lover(r,c,label, &WEST);
    }
    //bottom
    for label in C + R..2 * C + R {
        let r = 1 + R;
        let c = 2 * C + R - label;
        add_lover(r,c,label, &NORTH);
    }
    //left
    for label in 2 * C + R..2 * (R + C) {
        let r = 2 * (R + C) - label;
        let c = 0;
        add_lover(r,c,label, &EAST);
    }

    debug!("Grid\n{:#.3?}\n", grid);

    let mut matches:Vec<LoverPair> = Vec::new();
    for matched in lover_pairings.chunks_exact(2) {
        let L1_num = matched[0];
        let L2_num = matched[1];
        let L1 = lovers.iter().find(|lov| lov.number == L1_num).unwrap();
        let L2 = lovers.iter().find(|lov| lov.number == L2_num).unwrap();

        matches.push( LoverPair{ L1:L1.clone(), L2:L2.clone(), distance: L1.location.manhat_distance(&L2.location)} );
    }

    matches.sort_by( |a, b| a.distance.cmp(&b.distance).then(a.L1.number.cmp(&b.L1.number)) );

    let mut has_been_matched = BitVec::from_elem(2 * R * C + 1, false);

    'lover_pair_for: for lover_pair in matches.iter()
    {
        debug!("Matching {} and {}.  Distance={}", lover_pair.L1.number, lover_pair.L2.number, lover_pair.distance);

        let starting_location = GardenLocation{
            grid_loc: lover_pair.L1.location.clone() + lover_pair.L1.initial_direction,
            entry_dir: lover_pair.L1.initial_direction.clone()
        };
        let target_location = GardenLocation{
            grid_loc: lover_pair.L2.location.clone(),
            entry_dir: lover_pair.L2.initial_direction.clone() * -1
        };

        //used for distance calculations
        let target_grid_loc = lover_pair.L2.location.clone() + lover_pair.L2.initial_direction;

        let mut prev : HashMap<GardenLocation, GardenLocation> = HashMap::new();
        //Use a binary heap using -manhattan distance from target, GardenLocation
        let mut heap : BinaryHeap<HeapNode> = BinaryHeap::new();

        heap.push( HeapNode{
            loc: starting_location,
            distance_to_target: target_grid_loc.manhat_distance(&starting_location.grid_loc)
        });

        while let Some(heap_node) = heap.pop() {

            if heap_node.loc == target_location {
                debug!("Found target!");

                let mut prev_loc = heap_node.loc;
                while prev_loc != starting_location {
                    let cur_loc = prev_loc;
                    prev_loc = prev[&cur_loc];

                    if next_dir_loc_2(&prev_loc, true) == cur_loc {
                        assert_ne!(grid[ &prev_loc.grid_loc ], "\\".to_string());
                        grid[ &prev_loc.grid_loc ] = "/".to_string();
                    } else if next_dir_loc_2(&prev_loc, false) == cur_loc {
                        assert_ne!(grid[ &prev_loc.grid_loc ], "/".to_string());
                        grid[ &prev_loc.grid_loc ] = "\\".to_string();
                    } else {
                        panic!("Inconsistent prev location");
                    }
                }


                debug!("Grid\n{:#.3?}\n", grid);

                continue 'lover_pair_for;
            }

            debug!("Processing heap node row: {} col: {} direction: {:?} dist to target: {}",
            heap_node.loc.grid_loc.r(), heap_node.loc.grid_loc.c(),
                heap_node.loc.entry_dir, heap_node.distance_to_target);


            if heap_node.loc.grid_loc.r() == 0 ||
                heap_node.loc.grid_loc.r() == R as i64 + 1 ||
                heap_node.loc.grid_loc.c() == 0 ||
                heap_node.loc.grid_loc.c() == C as i64 + 1 {
                //we are out of bounds in the arms of another lover
                continue;
            }

            let grid_contents = &grid[&heap_node.loc.grid_loc];

            if grid_contents != "\\" {
                let loc = next_dir_loc_2(&heap_node.loc, true);
                let next_heap_node = HeapNode{
                    loc,
                    distance_to_target: target_grid_loc.manhat_distance(&loc.grid_loc)
                };
                if !prev.contains_key(&next_heap_node.loc) {
                    heap.push(next_heap_node);
                    prev.insert(next_heap_node.loc, heap_node.loc);
                }
            }

            if grid_contents != "/" {
                let loc = next_dir_loc_2(&heap_node.loc, false);
                let next_heap_node = HeapNode{
                    loc,
                    distance_to_target: target_grid_loc.manhat_distance(&loc.grid_loc)
                };
                if !prev.contains_key(&next_heap_node.loc) {
                    heap.push(next_heap_node);
                    prev.insert(next_heap_node.loc, heap_node.loc);
                }
            }
        }

        return "IMPOSSIBLE".to_string();

    }




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