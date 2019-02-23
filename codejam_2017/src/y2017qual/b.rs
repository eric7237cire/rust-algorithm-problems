use codejam::util::codejam::run_cases;
use std::io::Write;
//Adhoc / digits / integers

pub fn solve_all_cases()
{
    run_cases(
        &["B-small-practice", "B-large-practice"],
        "y2017qual",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let s = reader.read_string();

                if case_no != 1 {
                    //        continue;
                }

                println!("Solving case {}", case_no);

                writeln!(buffer, "Case #{}: {}", case_no, solve(&s)).unwrap();
            }
        },
    );
}

/*
Basically, if we have a non decreasing number

132  we need to find the left most max and decrease it
and make everything else 9s

so
1[3]2 ==> 129

222222[5]1 ==> 1999999
*/
fn solve(n_str: &str) -> usize
{
    debug!("n_str={}", n_str);
    let mut number: Vec<u8> = n_str
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    // Keep track of first instance of max digit
    let mut max_digit_pos: Option<usize> = None;
    for pos in 0..number.len() {
        let digit = number[pos];
        match max_digit_pos {
            Some(max_d_pos) if digit > number[max_d_pos] => max_digit_pos = Some(pos),
            None => max_digit_pos = Some(pos),
            _ => (),
        }
        let max_digit_pos = max_digit_pos.unwrap();
        if digit >= number[max_digit_pos] {
            continue;
        }

        number[max_digit_pos] -= 1;
        for n in number.iter_mut().skip(max_digit_pos + 1) {
            *n = 9;
        }
    }

    number
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<usize>()
        .unwrap()
}
