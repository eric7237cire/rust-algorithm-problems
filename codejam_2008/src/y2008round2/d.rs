use codejam::util::bitvec64::BitVec64;
use codejam::util::codejam::run_cases;
use std::cmp::min;
use std::i32;
use std::io::Write;

/*
Hamiltonian path
Permutations
*/
pub fn solve_all_cases()
{
    run_cases(
        &["D-small-practice", "D-large-practice"],
        "y2008round2",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let k = reader.read_int();
                let s: Vec<char> = reader.read_string().chars().collect();

                if case_no != 1 {
                    //        continue;
                }

                println!("Solving case {}", case_no);

                writeln!(buffer, "Case #{}: {:.6}", case_no, solve(k, s.as_slice())).unwrap();
            }
        },
    );
}

//from node x, travel to y visiting all nodes in mask.  x and y do not have to be in mask
fn travel(
    x: usize,
    y: usize,
    k: usize,
    mask: BitVec64,
    edge_cost: &[Vec<i32>],
    memo: &mut Vec<Vec<Vec<i32>>>,
) -> i32
{
    let mut ans: i32 = memo[x][y][mask.data];

    if ans != -1 {
        return ans;
    }

    if mask.data == 0 {
        ans = edge_cost[x][y];
        memo[x][y][mask.data] = ans;
        return ans;
    }

    ans = i32::MAX;
    for i in 0..k {
        if mask.get(i) {
            ans = min(
                ans,
                edge_cost[x][i]
                    + travel(
                        i,
                        y,
                        k,
                        BitVec64::with_val(mask.data ^ 1 << i),
                        edge_cost,
                        memo,
                    ),
            );
        }
    }
    memo[x][y][mask.data] = ans;
    return ans;
}

fn solve(k: usize, s: &[char]) -> i32
{
    assert!(s.len() % k == 0);

    //translating bmerry's solution
    let mut normal = vec![vec![0; k]; k];
    let mut cross = vec![vec![0; k]; k];

    for block_start in (0..s.len()).step_by(k) {
        for col_a in 0..k {
            for col_b in 0..k {
                if s[block_start + col_a] != s[block_start + col_b] {
                    normal[col_a][col_b] += 1;
                }
                if block_start >= k && s[block_start - k + col_a] != s[block_start + col_b] {
                    cross[col_a][col_b] += 1;
                }
            }
        }
    }

    let mut memo = vec![vec![vec![-1; 1 << k]; k]; k];
    let mut best = i32::MAX;
    for col_a in 0..k {
        for col_b in 0..k {
            if col_a == col_b {
                continue;
            }

            let mut mask = BitVec64::with_val((1 << k) - 1);
            mask.set(col_a, false);
            mask.set(col_b, false);
            best = min(
                best,
                travel(col_a, col_b, k, mask, normal.as_slice(), &mut memo) + cross[col_b][col_a],
            );
        }
    }

    //add change to 1st group (I think)
    best + 1
}