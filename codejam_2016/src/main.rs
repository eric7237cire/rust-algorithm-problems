

#![allow(non_snake_case)]
mod y2016qual;

extern crate codejam;


use codejam::util::log::init_log;

fn main()
{
    init_log();

    y2016qual::a::solve_all_cases();
    y2016qual::b::solve_all_cases();
    y2016qual::c::solve_all_cases();
    y2016qual::d::solve_all_cases();
}
