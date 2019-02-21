use codejam::util::codejam::run_cases;
use std::io::Write;

/*
Greedy
*/
pub fn solve_all_cases()
{
    run_cases(
        &["A-small-practice", "A-large-practice"],
        "y2008round1c",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                 /*On the first line we have the maximum number of letters
                 to place on a key (P), the number of keys available (K) and
                 the number of letters in our alphabet (L) all separated by single spaces.*/

                let input_nums = reader.read_num_line();
                assert_eq!(3, input_nums.len());

                /*The second line has L non-negative integers. Each number represents the frequency of a certain letter.
                */
                let mut freq = reader.read_num_line();

                assert_eq!(freq.len(), input_nums[2]);

                if case_no != 3 {
                    // continue;
                }

                println!("Solving case {}", case_no);

                writeln!(buffer, "Case #{}: {}", case_no,
                         solve(input_nums[0],
                               input_nums[1], &mut freq)).unwrap();
            }
        },
    );
}

fn solve(max_letters_per_key: usize, num_keys: usize, freq: &mut Vec<usize>) -> usize
{
    freq.sort();
    freq.reverse();

    let mut total_cost = 0;
    let mut  cost = 1;
    let mut keys_left = num_keys;

    for lf in freq.iter() {
        assert!(cost <= max_letters_per_key);
        total_cost += cost * lf;
        keys_left-=1;
        if keys_left == 0 {
            cost+=1;
            keys_left = num_keys;
        }
    }

    total_cost
}
