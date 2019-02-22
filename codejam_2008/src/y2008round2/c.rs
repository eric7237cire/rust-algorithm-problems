use codejam::util::codejam::run_cases;
use std::io::Write;


/*
Binary search
Absolute value & inequalities
Geometry
*/
pub fn solve_all_cases()
{
    run_cases(
        &["C-small-practice",
        //    "C-large-practice"
        ],
        "y2008round2",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {

                let num_ships = reader.read_int();

                let ships: Vec<Ship> = (0..num_ships).map(|_| {
                    let shp = reader.read_num_line();
                    assert_eq!(4, shp.len());
                    Ship { x: shp[0], y: shp[1], z: shp[2], p: shp[3] }
                }).collect();

                if case_no != 3 {
                    // continue;
                }

                println!("Solving case {}", case_no);

                writeln!(buffer, "Case #{}: {}", case_no, solve(ships.as_slice())).unwrap();
            }
        },
    );
}

struct Ship
{
    x: f64,
    y: f64,
    z: f64,
    p: f64
}

const SMALLEST_DIFF: f64 = 1e-7;

fn solve(ships: &[Ship]) -> f64
{
    let mut lb = 0.;
    let mut ub = 1_000_000.;

    while ub - lb > SMALLEST_DIFF {
        let mut valid = true;
        let y = (ub+lb) / 2.;
        /*
     x + y + z ≤ xi + yi + zi + piY
   x + y + z ≥ xi + yi + zi - piY
   x + y - z ≤ xi + yi - zi + piY
   x + y - z ≥ xi + yi - zi - piY
   x - y + z ≤ xi - yi + zi + piY
   x - y + z ≥ xi - yi + zi - piY
   -x + y + z ≤ -xi + yi + zi + piY
   -x + y + z ≥ -xi + yi + zi - piY
   */
        for s in ships.iter() {
            let a = s.x + s.y + s.z - s.p * y;
            let b = s.x + s.y + s.z + s.p* y;
            let c = s.x + s.y - s.z - s.p* y;
            let d = s.x + s.y - s.z + s.p* y;
            let e = s.x - s.y + s.z - s.p* y;
            let f = s.x - s.y + s.z + s.p* y;
            let g = -s.x + s.y + s.z - s.p* y;
            let h = -s.x + s.y + s.z + s.p* y;

            let i1 = [(a - h) as f64 / 2., (b - g) as f64 / 2.];
            let i2 = [(c - e) as f64 / 2., (d - f) as f64 / 2.];

            if !(i1[0] <= i2[1] && i2[0] <= i1[1]) {
                valid = false;
                break;
            }
        }

        if valid {
            ub = y;
        } else {
            lb = y;
        }
    }

    ub
}
