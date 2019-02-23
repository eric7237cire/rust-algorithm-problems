use codejam::util::codejam::run_cases;
use std::io::Write;
use std::thread;

/*
velocity
intersection
problem modeling/understanding
proof
*/
#[derive(Debug)]
struct Horse
{
    velocity: f64,
    start_pos: f64,
}

pub fn solve_all_cases()
{
    run_cases(
        &["A-small-practice", "A-large-practice"],
        "y2008round1B",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                //D & N
                let input: Vec<u64> = reader.read_num_line();
                let n = input[1];
                let mut horse = (0..n)
                    .map(|_| {
                        let input: Vec<u64> = reader.read_num_line();
                        Horse {
                            start_pos: input[0] as f64,
                            velocity: input[1] as f64,
                        }
                    })
                    .collect::<Vec<_>>();

                if case_no != 1 {
                    //        continue;
                }

                println!("Solving case {}", case_no);

                writeln!(
                    buffer,
                    "Case #{}: {}",
                    case_no,
                    solve(input[0] as f64, &mut horse)
                )
                .unwrap();
            }
        },
    );
}

fn solve(D: f64, horses: &mut Vec<Horse>) -> String
{
    //let mut horses = horses.clone();

    //Sort by starting position
    horses.sort_by(|h1, h2| h1.start_pos.partial_cmp(&h2.start_pos).unwrap());

    let mut cur_index = 0;

    while cur_index < horses.len() - 1 {
        let cur = &horses[cur_index];
        let next = &horses[cur_index + 1];
        if next.velocity >= cur.velocity {
            //anything that is faster won't affect the answer
            horses.remove(cur_index + 1);
            continue;
        }

        //Now make sure they intersect before D
        let inter_t = (cur.start_pos - next.start_pos) / (next.velocity - cur.velocity);
        let inter_p = cur.start_pos + cur.velocity * inter_t;

        if inter_p >= D {
            debug!("other horse finishes before: {:?} {:?}", cur, next);
            horses.remove(cur_index + 1);
            continue;
        }

        cur_index += 1;
    }

    //Only the last horse actually matters
    let hs = horses.last().unwrap();
    debug!("After processing, horse is {:?}.  V={:3}", hs, hs.velocity);

    let t = (D - hs.start_pos) / hs.velocity;
    let min_v = D / t;
    debug!(
        "After processing, horse is {:?}.  V={:3}.  V to intersect={:3}",
        hs, hs.velocity, min_v
    );

    format!("{:.6}\n", min_v)
}
