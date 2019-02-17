use codejam::util::codejam::run_cases;
use std::io::Write;

use codejam::util::vector_2d::Vector2d;
use std::f64;
use std::f64::consts::PI;

/*
Triangles
Basic geometry
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

                if case_no != 3 {
                    // continue;
                }

                println!("Solving case {}", case_no);

                writeln!(
                    buffer,
                    "Case #{}: {}",
                    case_no,
                    solve(
                        &Vector2d::with_val(floats[0], floats[1]),
                        &Vector2d::with_val(floats[2], floats[3]),
                        &Vector2d::with_val(floats[4], floats[5])
                    )
                )
                .unwrap();
            }
        },
    );
}

struct Line
{
    slope: Option<f64>,
    len: f64,
}

impl Line
{
    fn new(p1: &Vector2d<f64>, p2: &Vector2d<f64>) -> Line
    {
        Line {
            slope: if p1.x() == p2.x() {
                None
            } else {
                Some((p2.y() - p1.y()) / (p2.x() - p1.x()))
            },
            len: p1.pyth_distance(p2),
        }
    }

    fn is_parallel(&self, other: &Line) -> bool
    {
        self.slope == other.slope
    }
}

fn law_cosines(a: f64, b: f64, c: f64) -> f64
{
    //c2 = a2 + b2 – 2ab cos C
    //#puts a, b, c, ((c ** 2 - a ** 2 - b ** 2) / (-2 * a * b))
    ((c.powi(2) - a.powi(2) - b.powi(2)) / (-2. * a * b)).acos()
}

fn solve(p1: &Vector2d<f64>, p2: &Vector2d<f64>, p3: &Vector2d<f64>) -> String
{
    let ab = Line::new(p1, p2);
    let ac = Line::new(p1, p3);
    let bc = Line::new(p2, p3);

    if ab.is_parallel(&ac) || ab.is_parallel(&bc) || ac.is_parallel(&bc) {
        return "not a triangle".to_string();
    }

    let mut angles = Vec::new();
    angles.push(law_cosines(ab.len, ac.len, bc.len));
    angles.push(law_cosines(ab.len, bc.len, ac.len));
    angles.push(law_cosines(ac.len, bc.len, ab.len));

    let desc1 = if ab.len == ac.len || ab.len == bc.len || ac.len == bc.len {
        "isosceles"
    } else {
        "scalene"
    };

    let desc2 = if angles.iter().any(|&ang| (ang - PI / 2.).abs() < 0.00000001) {
        "right"
    } else if angles.iter().any(|&ang| ang > PI / 2.) {
        "obtuse"
    } else {
        "acute"
    };

    format!("{} {} triangle", desc1, desc2)
}
