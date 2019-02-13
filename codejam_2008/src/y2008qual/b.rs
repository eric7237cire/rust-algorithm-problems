//use bit_set::BitSet;
use codejam::util::codejam::run_cases;
use std::io::Write;
use std::usize;

/*
Simulation
Digits
*/
pub fn solve_all_cases()
{
    run_cases(
        &["B-small-practice", "B-large-practice"],
        "y2008qual",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let t = reader.read_int();

                let (na, nb) = reader.read_tuple_2();

                let read_times = |in_string : String| -> [usize;2] {
                    [
                        &in_string[0..2].parse::<usize>().unwrap() * 60 + &in_string[3..5].parse::<usize>().unwrap(),
                        &in_string[6..8].parse::<usize>().unwrap() * 60 + &in_string[9..11].parse::<usize>().unwrap()
                        ]};

                let arrivals : Vec<_> = (0..na).map(|_| read_times(reader.read_string()) ).collect();
                let departures : Vec<_> = (0..nb).map(|_| read_times(reader.read_string()) ).collect();

                if case_no != 3 {
                    // continue;
                }

                println!("Solving case {}", case_no);

                writeln!(
                    buffer,
                    "Case #{}: {}",
                    case_no,
                    solve(t, arrivals.as_slice(), departures.as_slice())
                )
                .unwrap();
            }
        },
    );
}

fn solve(turn_around_time: usize, arrivals: &[ [usize;2] ], departures: &[ [usize;2] ]) -> usize
{
    debug!("{:?}", arrivals[0]);

    (i < na ? station_a_events : station_b_events).push(TrainEvent.new(startTime, -1))
		(i < na ? station_b_events : station_a_events).push(TrainEvent.new(add_time(doneTime, turnAroundTime), 1))
   0
}
