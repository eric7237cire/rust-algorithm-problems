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

    fn sum_to(&self, to: usize) -> i64
    {
        //beginning of last row, leaves
        let start = 1 << (self.num_levels -1);
        let mut i = to + start;
        let mut sum = self.data[i];
        while i > 1 {
            println!("I is {}  {:0>width$b}", i, i, width=7);

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

    fn test_random()
    {
        let mut rng: StdRng = SeedableRng::seed_from_u64(42);

        let num_elems_gen = Uniform::from(1..10usize);
        let values_gen = Uniform::from(-10..10i64);

        for _ in 0..100 {
            let size = num_elems_gen.sample(&mut rng);
            let mut bt = BinaryTree::new(size);

            let mut check = vec![ 0; size];

            for _ in 0..100 {
                let pos = rng.gen_range(0, size);
                let val = values_gen.sample(&mut rng);

                check[pos] = val;
            }
        }
}
