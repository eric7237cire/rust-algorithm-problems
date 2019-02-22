use codejam::util::codejam::run_cases;
use std::cmp::Ordering::*;
use std::f64;
use std::io::Write;

/*
Binary search
Absolute value & inequalities
Geometry
*/
pub fn solve_all_cases()
{
    run_cases(
        &[
            "C-small-practice",
            "C-large-practice"
        ],
        "y2008round2",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let num_ships = reader.read_int();

                let ships: Vec<Ship> = (0..num_ships)
                    .map(|_| {
                        let shp = reader.read_num_line();
                        assert_eq!(4, shp.len());
                        Ship {
                            x: shp[0],
                            y: shp[1],
                            z: shp[2],
                            p: shp[3],
                        }
                    })
                    .collect();

                if case_no != 9 {
                    // continue;
                }

                println!("Solving case {}", case_no);

                writeln!(buffer, "Case #{}: {:.6}", case_no, solve(ships.as_slice())).unwrap();
            }
        },
    );
}

#[derive(Debug)]
struct Ship
{
    x: f64,
    y: f64,
    z: f64,
    p: f64,
}

const SMALLEST_DIFF: f64 = 1e-7;

fn fmin(a: f64, b: f64) -> f64
{
    if a.partial_cmp(&b).unwrap() == Greater {
        b
    } else {
        a
    }
}

fn fmax(a: f64, b: f64) -> f64
{
    if a.partial_cmp(&b).unwrap() == Less {
        b
    } else {
        a
    }
}

fn solve(ships: &[Ship]) -> f64
{
    let mut lb = 0.;
    let mut ub = 10_000_000.;

    while ub - lb > SMALLEST_DIFF {
        let mut max_a = f64::MIN;
        let mut min_b = f64::MAX;
        let mut max_c = f64::MIN;
        let mut min_d = f64::MAX;
        let mut max_e = f64::MIN;
        let mut min_f = f64::MAX;
        let mut max_g = f64::MIN;
        let mut min_h = f64::MAX;

        let mut valid ;
        let y = (ub + lb) / 2.;
        debug!("Trying power = {}.  # of ships: {}", y, ships.len());
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
        let mut x_interval_min = f64::MIN;
        let mut x_interval_max = f64::MAX;
        let mut y_interval_min = f64::MIN;
        let mut y_interval_max = f64::MAX;
        let mut z_interval_min = f64::MIN;
        let mut z_interval_max = f64::MAX;

        for s in ships.iter() {
            let a = s.x + s.y + s.z - s.p * y;
            let b = s.x + s.y + s.z + s.p * y;
            let c = s.x + s.y - s.z - s.p * y;
            let d = s.x + s.y - s.z + s.p * y;
            let e = s.x - s.y + s.z - s.p * y;
            let f = s.x - s.y + s.z + s.p * y;
            let g = -s.x + s.y + s.z - s.p * y;
            let h = -s.x + s.y + s.z + s.p * y;

            assert!(a <= b);
            assert!(c <= d);
            assert!(e <= f);
            assert!(g <= h);

            max_a = fmax(max_a, a);
            min_b = fmin(min_b, b);
            max_c = fmax(max_c, c);
            min_d = fmin(min_d, d);
            max_e = fmax(max_e, e);
            min_f = fmin(min_f, f);
            max_g = fmax(max_g, g);
            min_h = fmin(min_h, h);

            /*
                        debug!("A = {}", a);
                        debug!("B = {}", b);
                        debug!("C = {}", c);
                        debug!("D = {}", d);
                        debug!("E = {}", e);
                        debug!("F = {}", f);
                        debug!("G = {}", g);
                        debug!("H = {}", h);
            */

            let x_i1 = [(a - h) as f64 / 2., (b - g) as f64 / 2.];
            let x_i2 = [(c + e) as f64 / 2., (d + f) as f64 / 2.];

            let y_i1 = [(a - f) as f64 / 2., (b - e) as f64 / 2.];
            let y_i2 = [(c + g) as f64 / 2., (d + h) as f64 / 2.];

            let z_i1 = [(a - d) as f64 / 2., (b - c) as f64 / 2.];
            let z_i2 = [(e + g) as f64 / 2., (f + h) as f64 / 2.];

            //let x = fmax(x_i1[0], x_i2[0]);

            /*
                      A - x ≤ y + z ≤ B - x
            G + x ≤ y + z ≤ H + x
            C - x ≤ y - z ≤ D - x
            -F + x ≤ y - z ≤ -E + x
            */
            /*        debug!("A - x {} <= y+z <= B - x {}", a-x, b-x);
            debug!("G + x {} <= y+z <= H + x {}", g+x, h+x);
            debug!("C - x {} <= y-z <= D - x {}", c-x, d-x);
            debug!("-F + x {} <= y-z <= -E + x {}", -f+x, -e+x);*/

            x_interval_min = fmax(x_interval_min, x_i1[0]);
            x_interval_min = fmax(x_interval_min, x_i2[0]);

            x_interval_max = fmin(x_interval_max, x_i1[1]);
            x_interval_max = fmin(x_interval_max, x_i2[1]);

            y_interval_min = fmax(y_interval_min, y_i1[0]);
            y_interval_min = fmax(y_interval_min, y_i2[0]);

            y_interval_max = fmin(y_interval_max, y_i1[1]);
            y_interval_max = fmin(y_interval_max, y_i2[1]);

            z_interval_min = fmax(z_interval_min, z_i1[0]);
            z_interval_min = fmax(z_interval_min, z_i2[0]);

            z_interval_max = fmin(z_interval_max, z_i1[1]);
            z_interval_max = fmin(z_interval_max, z_i2[1]);
            /*
            if !(i1[0] <= i2[1] && i2[0] <= i1[1]) {
                debug!("Invalid!");
                valid = false;
                break;
            }*/
        }

        debug!("Interval X min {} max {}", x_interval_min, x_interval_max);
        debug!("Interval Y min {} max {}", y_interval_min, y_interval_max);
        debug!("Interval Z min {} max {}", z_interval_min, z_interval_max);

        let a = max_a;
        let b = min_b;
        let c = max_c;
        let d = min_d;
        let e = max_e;
        let f = min_f;
        let g = max_g;
        let h = min_h;

        //debug!("Overall x interval {:?} and {:?}", x_i1, x_i2);
        let x_i1 = [(a - h) as f64 / 2., (b - g) as f64 / 2.];
        let x_i2 = [(c + e) as f64 / 2., (d + f) as f64 / 2.];

        let y_i1 = [(a - f) as f64 / 2., (b - e) as f64 / 2.];
        let y_i2 = [(c + g) as f64 / 2., (d + h) as f64 / 2.];

        let z_i1 = [(a - d) as f64 / 2., (b - c) as f64 / 2.];
        let z_i2 = [(e + g) as f64 / 2., (f + h) as f64 / 2.];

        debug!("Interval X 1 {:?} Interval 2 {:?} ", x_i1, x_i2);
        debug!("Interval Y 1 {:?} Interval 2 {:?} ", y_i1, y_i2);
        debug!("Interval Z 1 {:?} Interval 2 {:?} ", z_i1, z_i2);

        /*
        A ≤ x + y + z ≤ B
   C ≤ x + y - z ≤ D
   E ≤ x - y + z ≤ F
   G ≤ -x + y + z ≤ H
   */
        debug!("A {} ≤ x + y + z ≤ B {}", a, b);
        debug!("C {} ≤ x + y - z ≤ D {}", c, d);
        debug!("E {} ≤ x - y + z ≤ F {}", e, f);
        debug!("G {} ≤ -x + y + z ≤ H {}", g, h);

        valid = a <= b && c <= d && e <= f && g <= h;

        valid = valid &&
            x_i1[0] <= x_i2[1] && x_i2[0] <= x_i1[1] && x_i1[0] <= x_i1[1] && x_i2[0] <= x_i2[1];
        valid = valid
            && y_i1[0] <= y_i2[1]
            && y_i2[0] <= y_i1[1]
            && y_i1[0] <= y_i1[1]
            && y_i2[0] <= y_i2[1];
        valid = valid
            && z_i1[0] <= z_i2[1]
            && z_i2[0] <= z_i1[1]
            && z_i1[0] <= z_i1[1]
            && z_i2[0] <= z_i2[1];

        //valid = x_interval_min <= x_interval_max && y_interval_min <= y_interval_max && z_interval_min <= z_interval_max;

        if valid {
            debug!("Valid, upper bound is now {}", y);
            ub = y;
        } else {
            debug!("InValid, lower bound is now {}", y);
            lb = y;
        }
    }

    ub
}
