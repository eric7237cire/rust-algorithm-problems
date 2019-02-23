use codejam::util::codejam::run_cases;
use std::io::Write;
use codejam::util::grid::constants::*;
use codejam::util::vector_2d::Vector2d;

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
                    path.extend( reader.read_string_line().chunks_exact(2).map( |ins| {
                        let steps = &ins[0];
                        let repeat = &ins[1];
                        (steps.chars().collect(), repeat.parse().unwrap())
                    }
                    ));
                }

                if case_no != 3 {
                    // continue;
                }

                println!("Solving case {}", case_no);

                writeln!(
                    buffer,
                    "Case #{}: {}",
                    case_no,
                    solve(path.as_slice())
                )
                .unwrap();
            }
        },
    );
}

const GRID_WIDTH : usize = 6100;
const COORD_OFFSET : i32= 3001;

const directions : [Vector2d<i64>;4] = [NORTH, EAST, SOUTH, WEST];

fn solve(path: &[(Vec<char>, u32)]) -> u32
{
    let mut col_rng : Vec< [i32;2] > = vec![ [i32::MAX, i32::MIN]; GRID_WIDTH];
    let mut row_rng : Vec< [i32;2] > = vec![ [i32::MAX, i32::MIN]; GRID_WIDTH];

    let mut  head:usize = 0;

    for (i, (s, t)) in path.iter().enumerate()
        {
            let len = s.len();
            for rep in 0..t  {
                for ch in s.iter() {
                match *ch
                {
                     'L' => {
                         head = (directions.len() + head - 1) % directions.len();
                     }
                     'R' => {
                         head = (head + 1) % directions.len();
                     }
                    'F':
                    {
                        pnt nxt = cur + del[head];
                        int idx;
                        switch(head)
                        {
                            case 0:
                            case 2:
                            idx = min(cur.real(), nxt.real());
                            col_rng[idx].first = min(col_rng[idx].first, cur.imag());
                            col_rng[idx].second = max(col_rng[idx].second, cur.imag());
                            break;
                            case 1:
                            case 3:
                            idx = min(cur.imag(), nxt.imag());
                            row_rng[idx].first = min(row_rng[idx].first, cur.real());
                            row_rng[idx].second = max(row_rng[idx].second, cur.real());
                            break;
                        }
                        area += imag(conj(cur) * nxt);
                        cur = nxt;
                    }
                }
            }
        }
        }
        assert(cur == pnt(0, 0));

        int ans = 0;
        for (int i = -3001; i <= 3001; i++)
            for (int j = -3001; j <= 3001; j++)
                if ((j >= row_rng[i].first && j < row_rng[i].second)
                    || (i >= col_rng[j].first && i < col_rng[j].second))
                    ans++;
        ans -= llabs(area / 2);
        printf("Case #%d: %d\n", cas + 1, ans);
    }
    5
}
