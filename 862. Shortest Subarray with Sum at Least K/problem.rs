pub struct Solution {

}

use std::collections::VecDeque;

impl Solution {
    
    pub fn shortest_subarray(a: Vec<i32>, k: i32) -> i32 {
        
        assert!(k>0);

        let mut sum_before : Vec<i32> = Vec::new();
        sum_before.push(0);
        for x in a.iter()
        {
            let last_val : i32 = *sum_before.last_mut().unwrap() ;
            sum_before.push(*x + last_val );
        }

        let mut best_start = 0;
        let mut best_end = a.len()+1;
        let mut start_points: VecDeque<usize> = VecDeque::new();
        
        //sum is start <= index < stop
        
        let mut start : usize = 0;
        let debug = false;
        let n:usize = a.len() ;

        //sum is from start <= index <= stop
        for end in 0..n {
            
            let total_to_end = sum_before[end+1];

            if debug {
                println!("Starting loop.  End is {}, total is {}", end, total_to_end);
            }

            while start_points.len() > 0 && total_to_end - sum_before[start_points[0]] >= k // adjust start
            {
                start = start_points.pop_front().unwrap();
                assert!(3>4);
            }

            if debug {
                println!("Start = {} End is {}, total is {}.  start_points={:?}", start, end, total_to_end, start_points);
            }
         
            if total_to_end - sum_before[start] >= k && end-start < best_end-best_start
            {
                best_start = start;
                best_end = end;
            }
            
            while start_points.len() > 0 && total_to_end <= sum_before[*start_points.back().unwrap()] // remove bad candidates
            {
                start_points.pop_back();
            }

            if debug {
                println!("After removing bad candidates.  Start = {} End is {}, total is {}.  start_points={:?}", start, end, total_to_end, start_points);
            }

            start_points.push_back(end+1) // end+1 is a new candidate
        }
        if best_end == a.len() + 1 {
            return -1;
        } else {
            return (best_end - best_start + 1) as i32;
        }

    }
}


fn main() {
    
    let checks = [
        ( (vec![1,2], 4), -1),
      ( (vec![39353,64606,-23508,5678,-17612,40217,15351,-12613,-37037,64183,68965,-19778,-41764,-21512,17700,-23100,77370,64076,53385,30915,18025,17577,10658,77805,56466,-2947,29423,50001,31803,9888,71251,-6466,77254,-30515,2903,76974,-49661,-10089,66626,-7065,-46652,84755,-37843,-5067,67963,92475,15340,15212,54320,-5286],

207007), 4),
( (vec![
    12,17,26,72,93,
    95,-46,66,-38,-18,
    83,95,75,8,93,
    7,25,16,67,-19],
413), 9 ),
        ( (vec![77,19,35,10,-14], 19), 1 ),
   ( (vec![-34,37,51,3,-12,-50,51,100,-47,99,34,
   14,-13,89,31,-14,-44,23,-38,6],
   151), 2),
    ( (vec![1], 1), 1 ),
    ( (vec![1,2], 4), -1 ) ,
    ( (vec![2,-1,2], 3), 3 ) ,
    ( (vec![2,3,2,1,-1,2,4], 6), 2 ) , 
    ( (vec![1,2,3,3], 6), 2 ) , 
    ( (vec![-2,3,2,1,-1,2,4], 7), 5 )
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
       // break;
    }
}