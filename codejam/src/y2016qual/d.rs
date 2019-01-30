use crate::algo::graph::flow2::Flow;
use crate::algo::prime::sieve::SieveOfAtkin;

use crate::util::codejam::run_cases;
use bit_set::BitSet;
use bit_vec::BitVec;
use byteorder::{BigEndian, ByteOrder, LittleEndian, NativeEndian, WriteBytesExt};
use num_bigint::BigUint;
use num_traits::*;
use rand::{thread_rng, Rng};
use std::cmp::max;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::Write;
use std::mem;
use std::usize;

use std::thread;

/*
Greedy alogrithm
*/
pub fn solve_all_cases()
{
    run_cases(
        &["D-small-practice", 
        //"C-large-practice"
        ],
        "y2016qual",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let (K, C, S) = reader.read_tuple_3();

                if case_no != 1 {
                   // continue;
                }

                println!("Solving case {}", case_no);

                writeln!(buffer, "Case #{}: {}", case_no, 
                if let Some(ans) =solve(K, C, S) {
                    ans.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" ") } else { "IMPOSSIBLE".to_string()}  ).unwrap();
            }
        },
    );
}

fn solve(K: u16, C: u16, S: u16) -> Option<Vec<u64>>
{
    if C * S < K {
        return None;
    }

    //we just need positions whose digits in base K match
    Some((0..K).collect::<Vec<_>>().chunks(C as usize).map( |digits| {
        let mut pos = 0u64;
        let mut base = 1u64;
        for d in digits {
            pos += *d as u64 * base;
            base *= K as u64;
        }
        pos + 1
    }).collect())
}


#[cfg(test)]
mod test_2016_qual_d
{
    use super::*;

    fn generate_sequence(C: u8, initial_seq: &[char]) -> Vec<char>
    {
        let K = initial_seq.len();
        let mut seq_old = Vec::new();
        seq_old.extend(initial_seq.iter());

        let mut seq_new = Vec::new();
        for c in 0..C-1 {
            seq_new.clear();
            for ch in seq_old.iter() 
            {
                if *ch == 'G' {
                    seq_new.extend( "G".repeat( initial_seq.len() ).chars() );
                } else {
                    seq_new.extend( initial_seq.iter() );
                }
            }

            // println!("After {}, seq = {:?}", c, seq_new);
             println!("After {}, G count = {}, seq len = {}", c, seq_new.iter().filter(|&&c| c == 'G').count(), seq_new.len());

            mem::swap(&mut seq_old, &mut seq_new);
        }

        return seq_old;
    }

    fn convert_to_base(num: u64, base: u64) -> Vec<u8> {
        let mut ans = Vec::new();
        let mut num = num;
        while num > 0 {
            ans.push( (num % base) as u8 );
            num /= base;
        }

        ans
    }

    #[test]
    fn test_det_g()
    {
        let init_seq = ['G', 'L','L','L','L'];
        let C = 4;
        let test_seq = generate_sequence(C, &init_seq);

        assert_eq!(test_seq.len(), init_seq.len().pow(C as u32));
        for pos in 0..625usize {
            //convert pos to base 5
            let mut digits = convert_to_base(pos as u64, 5);
            if digits.len() < C as usize {
                digits.push(0);
            }
            let expected_char = if digits.contains(&0) {
                'G'
            } else {    
                'L'
            };

            assert_eq!(test_seq[pos], expected_char, "Pos = {} Digits = {:?}", pos, digits);
        }
    }

     #[test]
    fn test_det_g2()
    {
        let init_seq = ['L', 'L','G',];
        let C = 5;
        let test_seq = generate_sequence(C, &init_seq);

        assert_eq!(test_seq.len(), init_seq.len().pow(C as u32));
        for pos in 0..test_seq.len() {
            //convert pos to base 5
            let mut digits = convert_to_base(pos as u64, init_seq.len() as u64);
            if digits.len() < C as usize {
                digits.push(0);
            }
            let expected_char = if digits.contains(&2) {
                'G'
            } else {    
                'L'
            };

            assert_eq!(test_seq[pos], expected_char, "Pos = {} Digits = {:?}", pos, digits);
        }
    }
}