use codejam::util::codejam::run_cases;
//use itertools::Itertools;
use std::i64;
use std::io::Write;

/*
Dynamic programming
*/
pub fn solve_all_cases()
{
    run_cases(
        &["C-small-practice", "C-large-practice"],
        "y2008practice",
        |reader, buffer| {
            let mut fcomp = Fcomp::new();

            let t = reader.read_int();

            for case_no in 1..=t {
                let (f, d, b) = reader.read_tuple_3();

                println!("Solving case {}", case_no);

                let f_max = if let Some(f) = fcomp.find_f_max(d, b) {
                    f as i64
                } else {
                    -1
                };
                let d_min = fcomp.find_d_min(f, b);
                let b_min = fcomp.find_b_min(f, d);

                writeln!(buffer, "Case #{}: {} {} {}", case_no, f_max, d_min, b_min).unwrap();
            }
        },
    );
}

struct Fcomp
{
    cache: Vec<Vec<Option<u64>>>,
}
const B_LIMIT: u64 = 1000;
const D_LIMIT: u64 = 1000;
const F_MAX_LIMIT: u64 = 1 << 32; //  4294967296;

impl Fcomp
{
    fn new() -> Self
    {
        Fcomp {
            cache: vec![vec![None; B_LIMIT as usize]; D_LIMIT as usize],
        }
    }

    fn find_f_max(&mut self, d: u64, b: u64) -> Option<u64>
    {
        let mut ret_val: Option<u64> =
            if d < D_LIMIT && b < B_LIMIT && self.cache[d as usize][b as usize].is_some() {
                self.cache[d as usize][b as usize]
            } else if b == 1 {
                //#breaking 1 egg means we can figure out d levels
                Some(d)
            } else if b == 2 {
                //#Happens to be the sum formula, since you can organize it like 5(if break) + 4 + 3 + 2 + 1
                Some((d * (d + 1)) / 2)
            } else if b > 2 && d >= 2954 {
                None
            } else if b > 3 && d >= 568 {
                None
            } else if b > 4 && d >= 221 {
                None
            } else if b >= 6 && d > 122 {
                None
            } else if b > 16 && d >= 33 {
                None
            } else if d <= b {
                Some(2u64.pow((d) as u32) - 1)
            } else {
                let lhs = self.find_f_max(d - 1, b - 1);
                let rhs = self.find_f_max(d - 1, b);
                if lhs.is_none() || rhs.is_none() {
                    None
                } else {
                    Some(1 + lhs.unwrap() + rhs.unwrap())
                }
            };

        if ret_val.is_some() && ret_val.unwrap() > F_MAX_LIMIT {
            ret_val = None;
        }

        if d < D_LIMIT && b < B_LIMIT {
            //if self.cache[d][b].is_none() {
            self.cache[d as usize][b as usize] = ret_val;
            //}
        }
        ret_val
    }

    fn find_d_min(&mut self, f: u64, b: u64) -> u64
    {
        let mut d = 1;

        while let Some(f_check) = self.find_f_max(d, b) {
            if f_check >= f {
                break;
            }
            d += 1;
        }

        d
    }

    fn find_b_min(&mut self, f: u64, d: u64) -> u64
    {
        let mut b = 1;

        while let Some(f_check) = self.find_f_max(d, b) {
            if f_check >= f {
                break;
            }
            b += 1;
        }

        b
    }
}

#[cfg(test)]
mod tests
{
    use crate::y2008practice::c::Fcomp;
    use crate::y2008practice::c::F_MAX_LIMIT;

    #[test]
    fn test_f_max()
    {
        let mut f_comp = Fcomp::new();

        assert_eq!(Some(14), f_comp.find_f_max(4, 3));
        assert_eq!(Some(7), f_comp.find_f_max(3, 3));
        assert_eq!(Some(6), f_comp.find_f_max(3, 2));

        assert_eq!(Some(3), f_comp.find_f_max(3, 1));
    }

    #[test]
    fn tests()
    {
        test_stuff(3, 3, 3, Some(7), 2, 1);
        test_stuff(7, 5, 3, Some(25), 3, 2);
        test_stuff(1, 122, 6, Some(4_258_490_215), 1, 1);
        test_stuff(1, 82, 7, Some(4_181_044_987), 1, 1);
        test_stuff(1, 83, 7, None, 1, 1);
        test_stuff(1, 82, 8, None, 1, 1);
        test_stuff(1, 220, 5, Some(4_199_307_189), 1, 1);
        test_stuff(1, 221, 5, None, 1, 1);
        test_stuff(1, 220, 6, None, 1, 1);
    }

    #[test]

    fn test_max()
    {
        let mut f_comp = Fcomp::new();

        for d in 1..10 {
            println!("D is {}", d);
            for b in 1..10 {
                assert!(f_comp.find_f_max(d, b).unwrap_or(0) < F_MAX_LIMIT,);
            }
        }
    }

    fn test_stuff(f: u64, d: u64, b: u64, ans_f_max: Option<u64>, ans_d_min: u64, ans_b_min: u64)
    {
        let mut fcomp = Fcomp::new();

        let (fmax, dmin, bmin) = (
            fcomp.find_f_max(d, b),
            fcomp.find_d_min(f, b),
            fcomp.find_b_min(f, d),
        );

        assert_eq!(fmax, ans_f_max);
        assert_eq!(dmin, ans_d_min);
        assert_eq!(bmin, ans_b_min);
    }
}
