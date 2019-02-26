use codejam::util::codejam::run_cases;
use std::io::Write;
use std::usize;
use std::cmp::Ordering::Less;

/*
TODO
*/
pub fn solve_all_cases()
{
    run_cases(
        &["C-small-practice", "C-large-practice"],
        "y2008round_apac",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let in_str = reader.read_string_line();
                let m : usize = in_str[0].parse().unwrap();
                let p : f64 = in_str[1].parse().unwrap();
                let x : usize = in_str[2].parse().unwrap();

                if case_no != 3 {
                    //continue;
                }
                println!("Solving case {}", case_no);

                writeln!(
                    buffer,
                    "Case #{}: {:.7}",
                    case_no,
                    solve(m,p,x)
                )
                .unwrap();
            }
        },
    );
}

fn solve(num_rounds: usize, p: f64, x: usize) -> f64
{
    let mut round_prob = vec![ vec![0.0; 1 << (num_rounds+1)]; num_rounds+1];

    round_prob[0][1] = 1.;
    round_prob[0][0] = 0.;

  for m in 1..=num_rounds
  {
	  let last_round_max = (1 << m-1) + 1;
	  //Combine rounds
	  for high_index in 0..last_round_max
	  {
		  //Copy over rounds
		  round_prob[m][high_index * 2] = fmax(round_prob[m][high_index * 2], round_prob[m-1][high_index]);

		  for low_index in 0..high_index
		  {
			  let this_round_idx = high_index + low_index;
			  round_prob[m][this_round_idx] = fmax(round_prob[m][this_round_idx], p * round_prob[m-1][high_index] + (1.-p) * round_prob[m-1][low_index]);
		  }
	  }	  
  }
      
  round_prob[num_rounds][(x as f64 / (1000000.0 / (1 << num_rounds) as f64)) as usize]
  
}


fn fmax(a: f64, b: f64) -> f64
{
    if a.partial_cmp(&b).unwrap() == Less {
        b
    } else {
        a
    }
}
