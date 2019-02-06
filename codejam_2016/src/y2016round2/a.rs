use codejam::util::codejam::run_cases;

use itertools::Itertools;
use std::io::Write;

/*

*/
pub fn solve_all_cases()
{
    run_cases(
        &["A-small-practice",
         "A-large-practice"
        ],
        "y2016round2",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                

                let numbers = reader.read_num_line();
                assert_eq!(4, numbers.len());

                if case_no != 3 {
                    // continue;
                }

                //println!("Solving case {}", case_no);

                writeln!(buffer, "Case #{}: {}", case_no, solve(numbers[0], numbers[1],numbers[2],numbers[3])).unwrap();
            }
        },
    );
}
#[derive(Clone, Copy, Debug, PartialEq)]
enum Node {
    Rock,
    Paper,
    Scissors
}

use Node::*;

fn solve(N: usize, R: usize, P: usize, S: usize) -> String
{
    debug!("Solving N={}  R={} P={} S={}", N, R,P,S);
    //generate all 3 possibilites (R / P / S winning)

    //store tree in array [ 0 .. 1 2 ... 3 4 5 6 ... 7 8 9 10 11 12 13 14]
    // [ 0 2p+1 2p+2  
    let mut trees : Vec<Vec<Node>>  = Vec::new();
    
    trees.push( vec![Paper] );
    trees.push( vec![Rock] );    
    trees.push( vec![Scissors] );

    for tree in trees.iter_mut() 
    {
        for level in 0..N 
        {
            //0 goes 0 to 0
            //1 goes 1 to 2
            //2 goes 3 to 6
            //3 goes 7 to 14
            let stop = (1<< (level+1)) - 2;
            let start = stop + 1 - ( 1 << (level));
            for node in start..=stop
            {
                let children = match tree[node]
                {
                    Rock => [ Rock, Scissors],
                    Paper => [ Paper, Rock ],
                    Scissors => [ Paper, Scissors ]
                };

                tree.extend(children.iter().cloned());

            }
        }

        debug!("Tree is {:?}", tree);
    }

    //slice off the last row
    let stop = (1<< (1+N)) - 2;
    let start = stop + 1 - ( 1 << (N));

    assert_eq!(stop, trees[0].len()-1);
    assert_eq!(1<<N, stop-start+1);

    for tree in trees.iter()
    {
        let tree = &tree[start..];

        let r_count = tree.iter().filter( |&&n| n==Rock).count();
        let p_count = tree.iter().filter( |&&n| n==Paper).count();
        let s_count = tree.iter().filter( |&&n| n==Scissors).count();

        if r_count == R && p_count == P && s_count == S {
            return alpha_print( &tree.iter().map(|node| match node
                {
                    Rock => 'R',
                    Paper => 'P',
                    Scissors => 'S'
                }).collect::<Vec<char>>());
        }
    }

    "IMPOSSIBLE".to_string()
}

fn alpha_print( ans: &[char] ) -> String 
{
    if ans.len() == 1 {
        return ans[0].to_string();
    }

    let first_half = alpha_print(&ans[0..ans.len()/2]);
    let second_half = alpha_print(&ans[ans.len()/2..ans.len()]);
    
    if first_half < second_half {
        format!( "{}{}", first_half, second_half )
    } else {
        format!( "{}{}", second_half, first_half)
    }
}