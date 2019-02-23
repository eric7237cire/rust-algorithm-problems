use codejam::util::codejam::run_cases;
use codejam::util::grid::constants::*;
use codejam::util::vector_2d::Vector2d;
use std::cmp::max;
use std::cmp::min;
use std::i64;
use std::io::Write;

/*
Polygons
*/
pub fn solve_all_cases()
{
    run_cases(
        &["A-small-practice", "A-large-practice"],
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
const COORD_OFFSET: i64 = 3001;

const DIRECTIONS: [Vector2d<i64>; 4] = [NORTH, EAST, SOUTH, WEST];

fn solve(path: &[(Vec<char>, u32)]) -> i64
{
    let mut col_rng: Vec<[i64; 2]> = vec![[i64::MAX, i64::MIN]; GRID_WIDTH];
    let mut row_rng: Vec<[i64; 2]> = vec![[i64::MAX, i64::MIN]; GRID_WIDTH];

    let mut head: usize = 0;
    let mut cur: Vector2d<i64> = Default::default();
    let mut area: i64 = 0;

    for (s, t) in path.iter() {
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
                                idx = (min(cur.r(), nxt.r()) + COORD_OFFSET) as usize;
                                col_rng[idx][0] = min(col_rng[idx][0], cur.c());
                                col_rng[idx][1] = max(col_rng[idx][1], cur.c());
                            }
                            EAST | WEST => {
                                idx = (min(cur.c(), nxt.c()) + COORD_OFFSET) as usize;
                                let rr = row_rng.get_mut(idx).unwrap();

                                rr[0] = min(rr[0], cur.r());
                                rr[1] = max(rr[1], cur.r());
                            }
                            _ => panic!("Hmm"),
                        }
                        area += -cur.r() * nxt.r();
                        cur = nxt;
                    }
                    _ => panic!("hmmm"),
                }
            }
        }
    }

    assert_eq!(cur, Vector2d::with_val(0, 0));

    let mut ans = 0;
    for i in 0..GRID_WIDTH {
        let ii = (i as i64) - COORD_OFFSET;
        for j in 0..GRID_WIDTH {
            let jj = (j as i64) - COORD_OFFSET;

            if (jj >= row_rng[i][0] && jj < row_rng[i][1])
                || (ii >= col_rng[j][0] && ii < col_rng[j][1])
            {
                ans += 1;
            }
        }
    }
    ans -= (area / 2).abs();
    ans
}
