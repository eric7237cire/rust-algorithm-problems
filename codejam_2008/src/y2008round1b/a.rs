use codejam::util::codejam::run_cases;
use codejam::util::vector_2d::Vector2d;
use std::io::Write;

/*
Arithmetic
Proof
*/
pub fn solve_all_cases()
{
    run_cases(
        &["A-small-practice", "A-large-practice"],
        "y2008round1b",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                // n, A, B, C, D, x0, y0 and M

                let input_nums = reader.read_num_line();
                let n = input_nums[0] as usize;
                let a = input_nums[1];
                let b = input_nums[2];
                let c = input_nums[3];
                let d = input_nums[4];
                let x0 = input_nums[5];
                let y0 = input_nums[6];
                let m = input_nums[7];

                let mut x: u64 = x0;
                let mut y: u64 = y0;
                let mut trees = Vec::new();
                trees.push(Vector2d::with_val(x, y));

                for i in 1..n {
                    x = (a * x + b) % m;
                    y = (c * y + d) % m;
                    trees.push(Vector2d::with_val(x, y));
                }

                assert_eq!(trees.len(), n);

                if case_no != 3 {
                    // continue;
                }

                println!("Solving case {}", case_no);

                writeln!(buffer, "Case #{}: {}", case_no, solve(trees.as_slice())).unwrap();
            }
        },
    );
}

fn solve(trees: &[Vector2d<u64>]) -> u64
{
    let mut buckets = [0u64; 9];

    for t in trees.into_iter() {
        buckets[((t.r() % 3) * 3 + t.c() % 3) as usize] += 1;
    }

    let mut ret = 0;

    // The first case.
    for bi in buckets.iter().filter( |bi| **bi > 2) {
        // We use the formula for n choose 3 so that,
        // we don't use the same point twice or count
        // the same triangle more than once.
        ret += bi * (bi - 1) * (bi - 2) / 6;
    }
    // The third case.
    for (i, bi) in buckets.iter().enumerate() {
        for (j, bj) in buckets.iter().enumerate().skip(i) {
            for (k, bk) in buckets.iter().enumerate().skip(j) {
                if (((i / 3) + (j / 3) + (k / 3)) % 3 == 0)
                    && ((i % 3) + (j % 3) + (k % 3)) % 3 == 0
                {
                    ret += bi * bj * bk;
                }
            }
        }
    }
    ret
}
