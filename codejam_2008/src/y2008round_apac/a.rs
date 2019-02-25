use codejam::util::codejam::run_cases;
use codejam::util::vector_2d::Vector2d;
use itertools::Itertools;
use std::cmp::max;
use std::cmp::min;
use std::io::Write;
use std::isize;

/*
Geometry
Ranges
*/
pub fn solve_all_cases()
{
    run_cases(
        &["A-small-practice", "A-large-practice"],
        "y2008round_apac",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let n = reader.read_int();

                //h w, is bird
                let birds: Vec<(Vector2d<isize>, bool)> = (0..n)
                    .map(|_| {
                        let s = reader.read_string_line();
                        (
                            Vector2d::with_val(s[0].parse().unwrap(), s[1].parse().unwrap()),
                            s[2] == "BIRD",
                        )
                    })
                    .collect();

                let m: usize = reader.read_int();

                let unknown: Vec<Vector2d<isize>> = (0..m)
                    .map(|_| {
                        let (h, w) = reader.read_tuple_2();
                        Vector2d::with_val(h, w)
                    })
                    .collect();

                if case_no != 3 {
                    // continue;
                }

                println!("Solving case {}", case_no);

                writeln!(buffer, "Case #{}:\n{}", case_no, solve(&birds, &unknown)).unwrap();
            }
        },
    );
}
fn solve(known: &[(Vector2d<isize>, bool)], unknown: &[Vector2d<isize>]) -> String
{
    let mut row_range = [isize::MAX, isize::MIN];
    let mut col_range = [isize::MAX, isize::MIN];

    let birds: Vec<Vector2d<isize>> = known.iter().filter(|k| k.1).map(|k| k.0).collect();
    let non_birds: Vec<Vector2d<isize>> = known.iter().filter(|k| !k.1).map(|k| k.0).collect();

    for b in birds.iter() {
        row_range[0] = min(row_range[0], b.r());
        row_range[1] = max(row_range[1], b.r());
        col_range[0] = min(col_range[0], b.c());
        col_range[1] = max(col_range[1], b.c());
    }

    let mut ans: Vec<Option<bool>> = Vec::new();
    'outer_loop: for u in unknown.iter() {
        if row_range[0] <= u.r()
            && u.r() <= row_range[1]
            && col_range[0] <= u.c()
            && u.c() <= col_range[1]
        {
            ans.push(Some(true));
            continue 'outer_loop;
        }

        for nb in non_birds.iter() {
            //Does considering this unknown bird as a bird cause a contradiction?
            if min(row_range[0], u.r()) <= nb.r()
                && nb.r() <= max(row_range[1], u.r())
                && min(col_range[0], u.c()) <= nb.c()
                && nb.c() <= max(col_range[1], u.c())
            {
                ans.push(Some(false));
                continue 'outer_loop;
            }
        }

        //We don't know s**t
        ans.push(None);
    }

    ans.into_iter()
        .map(|a| match a {
            Some(true) => "BIRD",
            Some(false) => "NOT BIRD",
            None => "UNKNOWN",
        })
        .join("\n")
}
