use codejam::util::codejam::run_cases;
use codejam::util::grid::constants::*;
use codejam::util::vector_2d::Vector2d;
use std::cmp::max;
use std::cmp::min;
use std::isize;
use std::io::Write;
use itertools::Itertools;

/*
Polygons
*/
pub fn solve_all_cases()
{
    run_cases(
        &["A-small-practice",
            "A-large-practice"
        ],
        "y2008round3",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let l = reader.read_int();

                let mut path: Vec<(Vec<char>, u32)> = Vec::new();

                while path.len() < l {
                    path.extend(reader.read_string_line().chunks_exact(2).map(|ins| {
                        let steps = &ins[0];
                        let repeat = &ins[1];
                        (steps.chars().collect(), repeat.parse().unwrap())
                    }));
                }

                if case_no != 3 {
                    // continue;
                }

                println!("Solving case {}", case_no);

                writeln!(buffer, "Case #{}: {}", case_no, solve(path.as_slice())).unwrap();
            }
        },
    );
}

const GRID_WIDTH: usize = 6100;
const COORD_OFFSET: isize = 3001;

const DIRECTIONS: [Vector2d<isize>; 4] = [NORTH, EAST, SOUTH, WEST];

fn solve(path: &[(Vec<char>, u32)]) -> isize
{
    //index is a row
    let mut col_rng: Vec<[isize; 2]> = vec![[isize::MAX, isize::MIN]; GRID_WIDTH];

    //index is a column
    let mut row_rng: Vec<[isize; 2]> = vec![[isize::MAX, isize::MIN]; GRID_WIDTH];

    let mut head: usize = 0;
    let mut cur: Vector2d<isize> = Default::default();
    let mut area: isize = 0;



    for (s, t) in path.iter() {
        debug!("S = {} repeat = {}", s.iter().join(""), t);

        let t = *t;
        for _rep in 0..t {
            for ch in s.iter() {
                match *ch {
                    'L' => {
                        head = (DIRECTIONS.len() + head - 1) % DIRECTIONS.len();
                    }
                    'R' => {
                        head = (head + 1) % DIRECTIONS.len();
                    }
                    'F' => {
                        let nxt = cur + &DIRECTIONS[head];
                        let idx;
                        match DIRECTIONS[head] {
                            NORTH | SOUTH => {
                                assert_eq!(cur.c(), nxt.c());

                                idx = (min(cur.r(), nxt.r()) + COORD_OFFSET) as usize;
                                let cc = col_rng.get_mut(idx).unwrap();

                                cc[0] = min(cc[0], cur.c());
                                cc[1] = max(cc[1], cur.c());

                                debug!("Col range of {} {} is {:?}", idx, idx as isize-COORD_OFFSET, cc);
                                //x1*y2 - x2*y1
                                //x(y2-y1)

                                //polygon formula
                                area += cur.c() * (nxt.r() - cur.r());
                            }
                            EAST | WEST => {
                                assert_eq!(cur.r(), nxt.r());

                                idx = (min(cur.c(), nxt.c()) + COORD_OFFSET) as usize;
                                let rr = row_rng.get_mut(idx).unwrap();

                                rr[0] = min(rr[0], cur.r());
                                rr[1] = max(rr[1], cur.r());

                                //y(x1-x2)

                                debug!("Col range of {} {} is {:?}", idx, idx as isize-COORD_OFFSET, rr);

                                area += cur.r() * (cur.c() - nxt.c());
                            }
                            _ => panic!("Hmm"),
                        }

                        cur = nxt;
                    }
                    _ => panic!("hmmm"),
                }
            }
        }
    }

    assert_eq!(cur, Vector2d::with_val(0, 0));

    let mut ans = 0;
    for col in 0..GRID_WIDTH {
        let col_with_offset = (col as isize) - COORD_OFFSET;
        for row in 0..GRID_WIDTH {
            let row_with_offset = (row as isize) - COORD_OFFSET;

            //basically any square that has had movement above/below or left/right is in the area
            //including the pockets
            if (row_with_offset >= row_rng[col][0] && row_with_offset < row_rng[col][1])
                || (col_with_offset >= col_rng[row][0] && col_with_offset < col_rng[row][1])
            {
                ans += 1;
            }
        }
    }
    ans -= (area / 2).abs();
    ans
}
