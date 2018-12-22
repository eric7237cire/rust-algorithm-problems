pub struct Solution {

}

impl Solution {
    
    pub fn shortest_subarray(a: Vec<i32>, k: i32) -> i32 {
        
        //sum is start <= index < stop
        let mut current_sum = 0;
        let mut start : usize = 0;
        let mut stop : usize = 0;
        let mut min_len : usize = a.len() + 2;
        let debug = true;
        let n:usize = a.len() ;

        for( stop=0; stop < n; ++stop) {
            
            if debug {
                println!("\nStarting loop sum {}, k={} Index {} to {}",
                    current_sum,
                    k, start, stop);
            }

            //Do we need to extend to the right
            while current_sum < k && stop < n 
            {
                if current_sum < 0 {
                    current_sum = 0;
                    start = stop;
                    continue;
                }

                current_sum += a[stop];
                stop += 1;
            }

            

            if debug {
                println!("\nMid loop sum {}, k={} Index {} to {}",
                    current_sum,
                    k, start, stop);
            } 

            if current_sum >= k {
                /* To find the start, since we can have negative numbers, count
                from stop - 1 until we get at least k*/
                //let mut sub_sum = 0;
                current_sum = 0;
                start = stop;

                //starting from stop-1, find greatest start where we have 
                //the sub array total >= k
                while current_sum < k
                {
                    start-=1;
                    current_sum += a[start];
                }
            
                if stop - start < min_len {
                    min_len = stop - start ; 
                }
            
                if debug {
                    println!("Mid ++ sum {}, k={} Index {} to {}",
                        current_sum,
                        k, start, stop);
                }

                //Now we move left up until we no longer have a solution
                while current_sum >= k && start < stop && start < n-1
                {
                    current_sum -= a[start];
                    start += 1;                    
                }
                
                assert!(start <= stop);
                
                if current_sum >= k {
                    break;
                }
            }

            if debug {
                println!("Current sum {}, k={} Index {} to {}",
                    current_sum,
                    k, start, stop);
            }

           
        }
        
        if min_len > n {
            return -1
        } else {
            return min_len as i32;
        }
    }
}


fn main() {
    
    let checks = [
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
        break;
    }
}