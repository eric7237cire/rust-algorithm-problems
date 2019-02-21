use codejam::util::codejam::run_cases;
use itertools::Itertools;
use std::cmp::min;
use std::io::Write;
use codejam::util::binary_sum_tree::BinarySumTree;

/*
Binary Interval tree using an array
*/
pub fn solve_all_cases()
{
    run_cases(
        &["C-small-practice", "C-large-practice"],
        "y2008round1c",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let k: usize = reader.read_int();

                let nums = reader.read_num_line();
                assert_eq!(nums[0], nums.len() - 1);

                if case_no != 3 {
                    // continue;
                }

                println!("Solving case {}", case_no);

                writeln!(
                    buffer,
                    "Case #{}: {}",
                    case_no,
                    solve(k, &nums[1..]).iter().join(" ")
                )
                .unwrap();
            }
        },
    );
}


fn solve(k: usize, indices: &[usize]) -> Vec<usize>
{
    let mut bt = BinarySumTree::new(k);

    for i in 0..k {
        bt.set(i, 1);
    }

    println!("After set");
    let mut deck = vec![0; k];

    let mut cur_pos = deck.len() - 1;
    let mut sum_to_current_pos = bt.sum_to(cur_pos);

    for card_no in 1..=k {
        let target_sum = 1 + (card_no as i64 + sum_to_current_pos - 1) % bt.sum();
        cur_pos = bt.lower_bound(target_sum);
        debug!("Target sum is {}.  Cur_pos {}", target_sum, cur_pos);
        assert_eq!(deck[cur_pos], 0);
        deck[cur_pos] = card_no;
        bt.set(cur_pos, 0);
        sum_to_current_pos = target_sum - 1;

        //debug!("Deck after {} is {:?}", card_no, deck);
        //            bt.debug_print();
    }

    indices.iter().map(|i| deck[i - 1]).collect()
}
