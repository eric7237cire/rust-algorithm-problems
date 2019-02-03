use codejam::util::codejam::run_cases;

use itertools::Itertools;
use std::io::Write;
use bit_vec::BitVec;

/*
Counting paths in a DAG
*/
pub fn solve_all_cases()
{
    run_cases(
        &[
            "B-small-practice",
            // "A-large-practice"
        ],
        "y2016round1C",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let (B, M) = reader.read_tuple_2();


                if case_no != 1 {
                    //continue;
                }

                println!("Solving case {}", case_no);

                writeln!(buffer, "Case #{}: {}", case_no, solve_brute_force(B, M)).unwrap();
            }
        },
    );
}

fn solve_brute_force(B: usize, M: usize) -> String
{
    //generate 0 .. n(n+1)/2 permutations
    let max_connections = B * (B + 1) / 2 - B;
    for perm in 0..1 << max_connections {
        let mut perm = perm;
        let mut perm_count = 0;

        let mut get_next_perm = || {
            perm_count += 1;
            let next_perm = perm & 1 > 0;
            perm >>= 1;
            next_perm
        };

        let mut edges: Vec<BitVec> = Vec::new();
        for v in 0..B  {
            let mut adj_list = BitVec::from_elem(B, false);
            for adj_v in v + 1..B {

                if get_next_perm() {
                    adj_list.set(adj_v, true);
                }

            }

            edges.push(adj_list);
        }

        assert_eq!(B, edges.len());

        let num_paths = count_paths(&edges);

        if num_paths == M {
            return format!("POSSIBLE\n{}", edges.iter().map( |bitvec|
                bitvec.iter().map( |b| if b {'1'} else {'0'}).join("")).join("\n") )
        } else {
            /*
            println!("not = target {}.  was {}\n{}",
                     M, num_paths,
                     edges.iter().map( |bitvec|
                bitvec.iter().map( |b| if b {'1'} else {'0'}).join("")).join("\n") )*/
        }

        assert_eq!(perm_count, max_connections);
    }

    format!("IMPOSSIBLE")
}

//https://www.geeksforgeeks.org/number-of-paths-from-source-to-destination-in-a-directed-acyclic-graph/
fn count_paths(edges: &[BitVec]) -> usize
{
    //assume 0...n is the topological order
    let mut dp = vec![0; edges.len()];

    //1 way to get to B from B
    dp[edges.len() - 1] = 1;

    for vertex in (0..edges.len()).rev() {
        for (adj_vertex, is_connected) in edges[vertex].iter().enumerate() {
            if !is_connected {
                continue;
            }
            dp[vertex] += dp[adj_vertex];
        }
    }

    return dp[0];
}
