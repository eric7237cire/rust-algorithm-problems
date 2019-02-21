use codejam::util::codejam::run_cases;
use itertools::Itertools;
use std::io::Write;
use std::cmp::min;
use std::cmp::max;

/*


*/
pub fn solve_all_cases()
{
    run_cases(
        &["B-small-practice", "B-large-practice"],
        "y2008round2",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let (n, m, a) = reader.read_tuple_3();

                println!("Solving case {}", case_no);

                writeln!(
                    buffer,
                    "Case #{}: {}",
                    case_no,
                    if let Some(ans) = solve(n, m, a) {
                        ans.iter().join(" ")
                    } else {
                        "IMPOSSIBLE".to_string()
                    }
                )
                .unwrap()
            }
        },
    );
}

fn solve(n: usize, m: usize, a: usize) -> Option<[usize;6]>
{
  let narrow = min(n,m);
  let wide = max(n,m);

  if (wide * narrow < A) {
    return None;
  }

  let mut n1 = a / wide;
  let w2 = wide;
  let mut w1=0;
    let mut n2 = 0;

  if A%wide == 0 {
    w1 = 0;
    n2 = 0;
  } else {
    n1+=1;
    n2 = 1;
    w1 = n1*w2 - A;

  }

    assert_eq!(A, abs(n1 * w2 - n2 * w1) );
  //          printf("Case #%d: Xmax %d Ymax %d tar %d\n", test_case+1, N, M, A);
  //          printf("Case #%d: 0 0 %d %d %d %d\n", test_case+1, x1,y1,x2,y2);
  if (wide == m) {
    assert!(w1 >= 0);
    assert!(w1 <= m);
    assert!(w2 >= 0);
    assert!(w2 <= m);
    assert!(n1 >= 0);
    assert!(n1 <= n);
    assert!(n2 >= 0);
    assert!(n2 <= n);
    return Some([0,0,n1,w1,n2,w2]);
  } else {
    assert!(w1 >= 0);
    assert!(w1 <= n);
    assert!(w2 >= 0);
    assert!(w2 <= n);
    assert!(n1 >= 0);
    assert!(n1 <= m);
    assert!(n2 >= 0);
    assert!(n2 <= m);

      return Some([0,0,w1,n1,w2,n2]);
  }
          //cout << abs(x1 * y2 - x2 * y1) << endl;



}
