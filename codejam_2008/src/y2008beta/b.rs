use codejam::util::codejam::run_cases;
use itertools::Itertools;
use std::io::Write;
use superslice::*;

/*
Triangles
*/
pub fn solve_all_cases()
{
    run_cases(
        &["B-small-practice", "B-large-practice"],
        "y2008beta",
        |reader, buffer| {
            let t = reader.read_int();

            for case_no in 1..=t {
                let products = reader.read_string_line();
                let prices = reader.read_num_line();

                if case_no != 3 {
                    // continue;
                }

                println!("Solving case {}", case_no);

                writeln!(buffer, "Case #{}: {}", case_no, solve(products, prices)).unwrap();
            }
        },
    );
}

fn lis(xs: &Vec<u8>) -> usize
{
    let mut lis = Vec::new();

    for i in 0..xs.len() {
        let it = lis.lower_bound(&xs[i]);
        if it > lis.len() {
            lis.push(xs[i]);
        } else {
            lis[it] = xs[i];
        }
    }

    lis.len()
}

fn solve(products: Vec<String>, prices: Vec<u8>) -> String
{
    let mut ret = Vec::new();
    let mut products = products;
    let mut prices = prices;

    let mut sps = products.clone();
    sps.sort();

    for sorted_product in sps.iter() {
        //j
        let non_sorted_index = products
            .iter()
            .position(|non_sorted_product| non_sorted_product == sorted_product).expect("Should exist");

        let mut xsp = Vec::new();
        let mut psp = Vec::new();

        xsp.extend_from_slice(&prices[0..non_sorted_index]);
        xsp.extend_from_slice(&prices[non_sorted_index + 1..]);

        for k in 0..products.len() {
            if k == non_sorted_index {
                continue;
            }
            psp.push(products[k].clone());
        }

        if lis(&prices) == lis(&xsp) {
            prices = xsp;
            products = psp;
            ret.push(sorted_product.clone());
        }
    }

    ret.iter().join(" ")
}
