//use bit_set::BitSet;
use codejam::util::codejam::run_cases;
use std::io::Write;
use std::usize;
use codejam::util::vector_2d::Vector2d;

/*
Triangles
*/
pub fn solve_all_cases()
{
    run_cases(
        &["A-small-practice", "A-large-practice"],
        "y2008beta",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let floats = reader.read_num_line();

                let search_engines: Vec<_> = (0..n).map(|_| reader.read_string()).collect();

                let q = reader.read_int();

                let queries: Vec<_> = (0..q).map(|_| reader.read_string()).collect();

                if case_no != 3 {
                    // continue;
                }

                println!("Solving case {}", case_no);

                writeln!(
                    buffer,
                    "Case #{}: {}",
                    case_no,
                    solve(search_engines.as_slice(), queries.as_slice())
                )
                .unwrap();
            }
        },
    );
}

struct Line {
    slope: Option<f64>,
    len: f64
}

impl Line {
    fn new(p1: &Vector2d<f64>, p2: &Vector2d<f64>) -> Line
    {
        Line {
            slope:
            if p1.x() == p2.x() {
                None
            } else {
                Some((p2.y() - p1.y()) / (p2.x() - p1.x()))
            },
            len: p1.pyth_distance(p2)
        }
    }

    fn is_parallel(&self, other: &Line) -> bool {
        self.slope == other.slope
    }
}


fn law_cosines(a: f64, b: f64, c: f64) -> f64
{
	//c2 = a2 + b2 – 2ab cos C
	//#puts a, b, c, ((c ** 2 - a ** 2 - b ** 2) / (-2 * a * b))
	( (c.powi(2) - a.powi(2) - b.powi(2)) / (-2 * a * b) ).acos()
}


fn solve(p1: &Vector2d<f64>, p2: &Vector2d<f64>, p3: &Vector2d<f64>) -> String
{
    let mut s_changes = 0;
    let mut cur_q = 0;

    while cur_q != queries.len() {
        //given 1 switch, how many queries can we get through by choosing the search engine that occurs the latest in the next
        //batch of queries
        let s_potential_progress = search_engines.iter().map(|search| {
            let idx = queries[cur_q..queries.len()]
                .iter()
                .position(|q| q == search);
            if let Some(idx) = idx {
                idx + cur_q
            } else {
                queries.len()
            }
        });

        cur_q = s_potential_progress.max().unwrap();
        s_changes += 1;
    }

    if s_changes > 0 {
        s_changes -= 1;
    }

    s_changes
}
