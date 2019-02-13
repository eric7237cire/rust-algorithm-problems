use codejam::util::codejam::run_cases;
use std::io::Write;
use std::f64::consts::PI;

/*

*/
pub fn solve_all_cases()
{
    run_cases(
        &["C-small-practice", "C-large-practice"],
        "y2008qual",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {

                let floats = reader.read_num_line();



                if case_no != 1 {
                    // continue;
                }

                println!("Solving case {}", case_no);

                let ans = solve(floats[0], floats[1], floats[2], floats[3], floats[4]);
                writeln!(
                    buffer,
                    "Case #{}: {}",
                    case_no, ans

                )
                .unwrap();
            }
        },
    );
}

fn circle(x:f64, r:f64) -> f64 {
    (r.powi(2) - x.powi(2)).sqrt()
}

fn integral_circle(x:f64, r: f64) -> f64 {
	let y = circle(x, r);
	(	x * y + ( x / y ).atan() * r.powi(2) ) / 2.
}

fn solve(fly_radius: f64, racket_radius : f64, t: f64, chord_radius: f64, gap_len:f64) -> f64
{
    /*
    dbg!( fly_radius );
	dbg!( racket_radius);
	dbg!( t) ; //Racket outer circle radius
	dbg!( chord_radius);
	dbg!( gap_len);
	*/

	if gap_len < 2. * fly_radius {
        return 1f64;
    }
	if fly_radius > racket_radius - t {
        return 1f64;
    }

    let total_area = PI * racket_radius.powi(2);
    let mut miss_area = 0f64;

    //Go through squares with x1 < x2 ; y1 < y2
    //These squares are where the fly can get through

    //To go from x1 to x2 or y1 to y2
    let next_position2_inc = gap_len - 2. * fly_radius;

    //To go from x2 to x1' or y2 to y1'
    let next_position1_inc = 2. * chord_radius + 2. * fly_radius;

    //The first x1 and y1
    let first_position = chord_radius + fly_radius;

    //The lenght of a full square + chords
    let square_length = next_position1_inc + next_position2_inc;

    //The actual radius we generally care about
    let inner_radius = racket_radius - t - fly_radius;

    //dbg!(inner_radius);

    //Go up chord + fly, when do wo intersect
    let x_stop = circle(chord_radius + fly_radius, inner_radius);

    //dbg!(x_stop);

    let mut x1 = first_position;

    //dbg!(x1);

    let num_cols = ( (x_stop-x1) / square_length).ceil() as u64;

    //Are that a fly can get through
    let regular_square_area = next_position2_inc * next_position2_inc;

    assert!(x1 <= x_stop);

    for col in 0..num_cols
    {
        let mut x2 = x1 + next_position2_inc;
        if col == num_cols - 1 && x_stop < x2 {
            x2 = x_stop
        }

        let mut y1 = first_position;
        let y_stop = circle(x1, inner_radius );

        let y_arc_intersection = circle(x2, inner_radius);

        let num_squares = ( (y_stop-y1) / square_length).ceil() as u64;

        //The real time saver.  We need to know the first y2 that is < y_arc_intersection
        //
        // y2_0 + square_length * lowest_arc_intersection_square_num < y_arc_intersection
        // square_length * lowest_arc_intersection_square_num < y_arc_intersection - y2_0
        // lowest_arc_intersection_square_num > (y_arc_intersection - y2_0) / square_length (dividing switches comparison)
        // lowest_arc_intersection_square_num == ceil ( ... )
        let lowest_arc_intersection_square_num = ( ( (y_arc_intersection - (y1 + next_position2_inc) ) / square_length).ceil() ) as u64;

        //Catch up y1
        y1 += lowest_arc_intersection_square_num as f64 * square_length ;

        //Init missed area with all the regular squares
        let mut slice_miss_area = (lowest_arc_intersection_square_num as f64) * regular_square_area;

        for square_num in lowest_arc_intersection_square_num..num_squares
        {
            let mut y2 = y1 + next_position2_inc;
            if square_num == num_squares - 1 && y_stop < y2 {
                y2 = y_stop;
            }
            debug!( "  y1:[{}] y2:[{}]", y1, y2);

            //This means an arc is going through the square
            let mut x1_arc_intersection = circle(y2, inner_radius);
            let mut x2_arc_intersection = circle(y1, inner_radius);

            debug!( "  X1 arc {} x2 arc {}", x1_arc_intersection, x2_arc_intersection);

            //take care of the curve
            x1_arc_intersection = if x1 > x1_arc_intersection { x1 } else { x1_arc_intersection };
            x2_arc_intersection = if x2 < x2_arc_intersection { x2 } else { x2_arc_intersection };

            slice_miss_area +=  (y2-y1) * (x2_arc_intersection-x1) ;

            let area_under_curve = integral_circle(x2_arc_intersection, inner_radius) - integral_circle(x1_arc_intersection, inner_radius);
            let rect_area = y2 * (x2_arc_intersection - x1_arc_intersection);
            let arc_area = rect_area - area_under_curve;

            //Subtract what the curve cut off
            slice_miss_area -= arc_area;

            y1 = y2 + next_position1_inc;

            //debug!( "  Slice Miss area: {} total area: {}", slice_miss_area,slice_total_area);
        }

        miss_area += slice_miss_area;
        x1 = x2 + next_position1_inc;
    }

    miss_area *= 4.;

    1. - miss_area / total_area
}
