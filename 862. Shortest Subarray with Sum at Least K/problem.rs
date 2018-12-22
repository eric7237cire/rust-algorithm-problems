pub struct Solution {

}

impl Solution {
    pub fn shortest_subarray(a: Vec<i32>, k: i32) -> i32 {
        4
    }
}


fn main() {
    
    let checks = [
    ( (vec![1], 1), 1 ),
    ( (vec![1,2], 4), -1 ) ,
    ( (vec![2,-1,2], 3), 3 ) 
    ];

    println!("Hello, world!");

    for check in checks.iter()
    {
        let solution_args = &check.0;
        let expected_ans = check.1;
        let actual_ans = Solution::shortest_subarray(
            solution_args.0.to_vec(),
            solution_args.1);
        if expected_ans != actual_ans {
            println!("Problem {} != {}", actual_ans, expected_ans);
        } else {
            println!("OK {} == {}", actual_ans, expected_ans);
        
        }
    }
}