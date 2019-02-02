#![allow(non_snake_case)]
mod y2016qual;
mod y2016round1A;
mod y2016round1B;

extern crate codejam;

#[macro_use]
extern crate itertools;

use codejam::util::log::init_log;

pub fn run_y2016qual()
{
    y2016qual::a::solve_all_cases();
    y2016qual::b::solve_all_cases();
    y2016qual::c::solve_all_cases();
    y2016qual::d::solve_all_cases();
}

pub fn run_y20161A()
{
    y2016round1A::a::solve_all_cases();
    y2016round1A::b::solve_all_cases();
    y2016round1A::c::solve_all_cases();
}

fn main()
{
    init_log();

    /*

    */

    //y2016round1B::a::solve_all_cases();
    y2016round1B::b::solve_all_cases();
    //y2016round1A::c::solve_all_cases();
}
