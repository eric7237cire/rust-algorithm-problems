use codejam::util::codejam::run_cases;
use itertools::Itertools;
use std::cmp::min;
use std::io::Write;

/*
Binary Interval tree using an array
*/
pub fn solve_all_cases()
{
    run_cases(
        &[
            "C-small-practice", // "C-large-practice"
        ],
        "y2008round1b",
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

//A binary tree represented in an array
struct BinaryTree
{
    data: Vec<i64>,
    //Number of elements, not the size of the tree
    num_elems: usize,
    num_levels: usize
}

impl BinaryTree
{
    fn new(num_elems: usize) -> Self
    {
        let mut levels = 1;
        while 1 << (levels-1) < num_elems {
            levels += 1;
        }
        BinaryTree {
            data: vec![0; 1 << levels ],
            num_elems,
            num_levels: levels
        }
    }

    fn set(&mut self, idx: usize, val: i64)
    {
        assert!(idx < self.num_elems);
        let mut i = idx + (1 << (self.num_levels -1));
        let old_val = self.data[i];
        self.data[i] = val;
        while i > 0 {
            //divide by 2
            i >>= 1;
            self.data[i] += val - old_val;
        }
    }

    //assumes sums are positive
    //returns smallest index such that sum(0..=index) >= target_sum
    fn lower_bound(&self, target_sum: i64) -> usize
    {
        let mut range_width = 1 << (self.num_levels - 1);
        //exclusive upper bound
        let mut range_to = range_width;

        if self.data[1] < target_sum {
            //fail with a sensible value
            return self.num_elems;
        }
        let mut cur_node = 1;
        let mut target_sum = target_sum;
        while range_width >= 2 {
            println!("Cur node {} target sum {} range width {} range to {} val @ cur_node {}",
            cur_node, target_sum, range_width, range_to, self.data[cur_node]);

            if self.data[cur_node] == target_sum {
                return min(self.num_elems-1, range_to-1);
            }

            range_width /= 2;

            //lhs
            if self.data[cur_node*2] >= target_sum {
                cur_node = cur_node * 2;
                assert!(range_width < range_to);
                range_to -= range_width;
            } else {
                //subtract lhs
                target_sum -= self.data[cur_node*2];
                cur_node = cur_node * 2 + 1;

                assert!(self.data[cur_node] >= target_sum);
            }


        }

        range_to-1
    }

    fn sum(&self) -> i64
    {
        self.data[1]
    }

    fn sum_to(&self, to: usize) -> i64
    {
        //beginning of last row, leaves
        let start = 1 << (self.num_levels -1);
        let mut i = to + start;
        let mut sum = self.data[i];
        while i > 1 {
            debug!("I is {}  {:0>width$b}", i, i, width=7);

            if i & 1 > 0 {
                sum += self.data[i - 1];
            }
            //divide by 2
            i >>= 1;
        }

        sum
    }

    fn debug_print(&self)
    {
        println!("Binary tree: {:?}", self.data);

        let mut i = 1;
        let mut level = 0;
        while i < self.data.len() {
            let stop = min(self.data.len() - i, 1 << level);
            println!("Level {} = {:?}", level, &self.data[i..i + stop]);
            level += 1;
            i += stop;
        }
    }
}

fn solve(k: usize, indices: &[usize]) -> Vec<usize>
{
    let mut bt = vec![0; k << 1];

    // (0) 000
    // (1) 001  (2)   010
    // (3) 011  (4) 100  (5) 101 (6) 110

    // (1) 001 [0..=7]
    // (2) 010 [0..=3] (3) 011 [4..=7]
    // (4) 100 [0..=1] (5) 101 [2..=3] (6) 110 [4..=5] (7) 111
    // (8) 1000 [0..=0] .... (15) 1111

    // query for up to index x
    // x=0 = 1000
    // x=1 = 0100
    // x=2 = 0100 + 1010
    // x=3 = 0010
    // x=4 = 0010 + 1000
    // x=5 = 110
    // x=6 = 0010 + 0110 + 1110
    // x=7 = 0001

    //parent is (i) / 2
    //left child is =(2*i);
    //and right_son=(2*i)+1;

    // query(0 to 7) = 4 + 2 + 1
    // query(0 to 5) = 4 + 1

    for i in k..bt.len() {
        //add last row
        bt[i] = 1;
        let mut p = i;
        while p > 0 {
            //divide by 2
            p >>= 1;
            bt[p] += 1;
        }
    }

    debug!("Binary tree: {:?}", bt);

    let mut i = 1;
    let mut level = 0;
    while i < bt.len() {
        let stop = min(bt.len() - i, 1 << level);
        debug!("Level {} = {:?}", level, &bt[i..i + stop]);
        level += 1;
        i += stop;
    }

    assert_eq!(bt[1], k);

    let mut deck = vec![0; k];

    let mut cur_pos = 0;
    for card_no in 1..k {
        //for sum, write in binary 1

    }

    vec![1, 2]
}

#[cfg(test)]
mod test_binary_tree
{
    use super::*;
    use rand::distributions::{Distribution, Uniform};
    use rand::prelude::StdRng;
    use rand::SeedableRng;
    use rand::Rng;


    #[test]
    fn test_bt()
    {
        let mut bt = BinaryTree::new(9);
                bt.debug_print();

        assert_eq!(5, bt.num_levels);
        bt.set(3, 4);
        bt.set(8, 1);
        bt.set(5, 3);
        bt.set(1, 7);

        bt.debug_print();

        assert_eq!(0, bt.sum_to(0));
        assert_eq!(7, bt.sum_to(1));
        assert_eq!(7, bt.sum_to(2));
        assert_eq!(11, bt.sum_to(3));
        assert_eq!(11, bt.sum_to(4));
        assert_eq!(14, bt.sum_to(5));
        assert_eq!(14, bt.sum_to(6));
        assert_eq!(14, bt.sum_to(7));
        assert_eq!(15, bt.sum_to(8));

        bt.set(0, -8);

        bt.debug_print();

        assert_eq!(-8, bt.sum_to(0));
        assert_eq!(-1, bt.sum_to(1));
        assert_eq!(-1, bt.sum_to(2));
        assert_eq!(3, bt.sum_to(3));
        assert_eq!(3, bt.sum_to(4));
        assert_eq!(6, bt.sum_to(5));
        assert_eq!(6, bt.sum_to(6));
        assert_eq!(6, bt.sum_to(7));
        assert_eq!(7, bt.sum_to(8));
    }

    #[test]
    fn test_random()
    {
        let mut rng: StdRng = SeedableRng::seed_from_u64(42);

        let num_elems_gen = Uniform::from(1..100usize);
        let values_gen = Uniform::from(-100..100i64);

        for _ in 0..10 {
            let size = num_elems_gen.sample(&mut rng);
            let mut bt = BinaryTree::new(size);

            let mut check = vec![0; size];

            for _ in 0..size {
                let pos = rng.gen_range(0, size);
                let val = values_gen.sample(&mut rng);

                check[pos] = val;
                bt.set(pos, val);

                for i in 0..size {
                    assert_eq!(check.iter().take(i + 1).sum::<i64>(),
                               bt.sum_to(i));
                }
            }
        }
    }

    #[test]
    fn test_small()
    {
        let mut bt = BinaryTree::new(1);
        bt.set(0, -3);
        assert_eq!(bt.sum_to(0), -3);

        bt.set(0, 7);
        assert_eq!(bt.sum_to(0), 7);
    }

    #[test]
    fn test_big()
    {
        let mut bt = BinaryTree::new(1_000_000);
        bt.set(0, -3);
        assert_eq!(bt.sum_to(0), -3);

        bt.set(1_000_000 - 1, 7);
        assert_eq!(bt.sum_to(1_000_000-1), 4);
    }

    #[test]
    fn test_lower_bound_exact() {
        let size = 100;
        let mut bt = BinaryTree::new(size);

        for i in 0..size {
            bt.set(i, (2*(i+1)) as i64);
        }

        for i in 0..size {
            //sum formula
            let target_sum = ((i+1)*(i+2)) as i64;
            println!("sum of 2+4+...+ for index {} should be {}",
            i, target_sum);
            assert_eq!(bt.lower_bound(target_sum), i as usize);
        }
    }

    #[test]
    fn test_lower_bound_inexact() {
        let size = 5;
        let mut bt = BinaryTree::new(size);

        for i in 0..size {
            bt.set(i, (2*(i+1)) as i64);
        }

        //2+4+6+8+10
        assert_eq!(bt.lower_bound(1), 0);
        assert_eq!(bt.lower_bound(3), 1);
        assert_eq!(bt.lower_bound(5), 1);
        assert_eq!(bt.lower_bound(7), 2);
        assert_eq!(bt.lower_bound(11), 2);
        assert_eq!(bt.lower_bound(13), 3);
        assert_eq!(bt.lower_bound(19), 3);
        assert_eq!(bt.lower_bound(21), 4);
        assert_eq!(bt.lower_bound(29), 4);

        assert_eq!(bt.lower_bound(30), 4);
        assert_eq!(bt.lower_bound(31), 5);
    }

}
