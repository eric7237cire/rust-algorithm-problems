use codejam::util::codejam::run_cases;
use std::io::Write;
use itertools::Itertools;


/*
*/
pub fn solve_all_cases()
{
    run_cases(
        &["C-small-practice", "C-large-practice"],
        "y2008beta",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let line1 = reader.read_string_line();
                let num_roads: u8 = line1[0].parse().unwrap();
                let starting_city = line1[1].clone();

                let roads: Vec<(String, String, u32)> = (0..num_roads).map(|_| {
                    let line = reader.read_string_line();
                    assert_eq!(3, line.len());
                    (line[0].clone(), line[1].clone(), line[2].parse().unwrap())
                }).collect();


                if case_no != 3 {
                    // continue;
                }

                println!("Solving case {}", case_no);

                writeln!(
                    buffer,
                    "Case #{}: {}",
                    case_no,
                    solve(
                        starting_city,
                        &roads
                    ).iter().map(|f| format!("{:.7}", f) ).join(" ")
                )
                .unwrap();
            }
        },
    );
}


fn solve(starting_city: String, roads: &[(String, String, u32)]) -> Vec<f64>
{
    vec![3.2, 1.9]
}
