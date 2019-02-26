use codejam::util::codejam::run_cases;
//use std::cmp::max;
//use itertools::Itertools;
use std::io::Write;
use std::mem::swap;

use permutohedron::LexicalPermutation;

/*
*/
pub fn solve_all_cases()
{
    run_cases(
        &["B-small-practice", "B-large-practice"],
        "y2016round3",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {


                if case_no > 3 {
                    // continue;
                }

                println!("Solving case {}", case_no);

                writeln!(
                    buffer,
                    "Case #{}: {}",
                    case_no,
                    solve() //solve_brute_force(K, &prob)
                )
                .unwrap();
            }
        },
    );
}

fn solve() -> i32
{
    3
}


#[cfg(test)]
mod test_2016_round3
{
    use permutohedron::LexicalPermutation;

    #[test]
    fn test_total_perms()
    {
        //if 1 prereq chain A->B, div 2
        //if len 3 prereq chai A->B->C ..O..O..O... div 3!

        let mut seq:Vec<usize> = (0..9).collect();
        assert_eq!(9, seq.len());

        let mut pos = seq.clone();

        let mut count = 0;

        let mut count_1 = 0;
        let mut count_2 = 0;
        loop {
            count += 1;

            for (i,v) in seq.iter().enumerate() {
                pos[*v] = i;
            }

            if pos[5] > pos[2] && pos[0] > pos[8] {
                count_1 += 1;
            }

            //enforce 8 2 5 and 7 0
            if pos[8] < pos[2] && pos[2] < pos[5] && pos[7] < pos[0] {
                count_2 += 1;
            }

            if !seq.next_permutation() {
                break;
            }
        }
        assert_eq!(count, (1..=9).product());
        assert_eq!(count / 4, count_1, "2 sequences of len 2");
        assert_eq!(count / (3*2 * 2), count_2, "1 seq len 3, 1 seq len 2");
    }
}
