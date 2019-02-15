//use bit_set::BitSet;
use codejam::util::codejam::run_cases;
use std::cmp::min;
use std::cmp::Ordering;
use std::io::Write;
use std::usize;

/*
Scan line
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

                let read_times = |in_string: String| -> [usize; 2] {
                    [
                        &in_string[0..2].parse::<usize>().unwrap() * 60
                            + &in_string[3..5].parse::<usize>().unwrap(),
                        &in_string[6..8].parse::<usize>().unwrap() * 60
                            + &in_string[9..11].parse::<usize>().unwrap(),
                    ]
                };

                let arrivals: Vec<_> = (0..na).map(|_| read_times(reader.read_string())).collect();
                let departures: Vec<_> =
                    (0..nb).map(|_| read_times(reader.read_string())).collect();

                if case_no != 1 {
                    // continue;
                }

                println!("Solving case {}", case_no);

                let ans = solve(t, arrivals.as_slice(), departures.as_slice());
                writeln!(buffer, "Case #{}: {} {}", case_no, ans[0], ans[1]).unwrap();
            }
        },
    );
}

#[derive(Eq, PartialEq)]
struct TrainEvent
{
    time: usize,
    num_train_change: i64,
}
impl Ord for TrainEvent
{
    fn cmp(&self, rhs: &TrainEvent) -> Ordering
    {
        self.time
            .cmp(&rhs.time)
            .then_with(|| rhs.num_train_change.cmp(&self.num_train_change))
    }
}

impl PartialOrd for TrainEvent
{
    fn partial_cmp(&self, other: &TrainEvent) -> Option<Ordering>
    {
        Some(self.cmp(other))
    }
}

fn find_required_trains(station_events: &mut Vec<TrainEvent>) -> i64
{
    debug!("find_required_trains");
    station_events.sort();

    //so as trains arrive (+1) they cannot affect the minimum
    let mut trains_min = 0;
    let mut trains = 0;

    for evt in station_events.iter() {
        debug!(
            "Train event at time {}.  Change {}",
            evt.time, evt.num_train_change
        );
        trains += evt.num_train_change;
        trains_min = min(trains, trains_min);
        debug!("Trains now {}.  min {}", trains, trains_min);
    }

    debug!("Done find_required_trains {} ", trains_min.abs());
    return trains_min.abs();
}

fn solve(turn_around_time: usize, a_to_b: &[[usize; 2]], b_to_a: &[[usize; 2]]) -> [i64; 2]
{
    debug!("{:?}", a_to_b[0]);

    let mut station_a_events = Vec::new();
    let mut station_b_events = Vec::new();

    for time in a_to_b.iter() {
        station_a_events.push(TrainEvent {
            time: time[0],
            num_train_change: -1,
        });
        station_b_events.push(TrainEvent {
            time: time[1] + turn_around_time,
            num_train_change: 1,
        });
    }
    for time in b_to_a.iter() {
        station_b_events.push(TrainEvent {
            time: time[0],
            num_train_change: -1,
        });
        station_a_events.push(TrainEvent {
            time: time[1] + turn_around_time,
            num_train_change: 1,
        });
    }

    [
        find_required_trains(&mut station_a_events),
        find_required_trains(&mut station_b_events),
    ]
}
