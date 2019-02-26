#![allow(non_snake_case)]
mod y2016qual;
mod y2016round1a;
mod y2016round1b;
mod y2016round1c;
mod y2016round2;
mod y2016round3;


extern crate codejam;

#[macro_use]
extern crate itertools;

#[macro_use]
extern crate log;

use codejam::util::log::init_log;

pub fn run_y2016_round_qual()
{
    y2016qual::a::solve_all_cases();
    y2016qual::b::solve_all_cases();
    y2016qual::c::solve_all_cases();
    y2016qual::d::solve_all_cases();
}

pub fn run_y2016_round_1a()
{
    y2016round1a::a::solve_all_cases();
    y2016round1a::b::solve_all_cases();
    y2016round1a::c::solve_all_cases();
}

pub fn run_y2016_round_1b()
{
    y2016round1b::a::solve_all_cases();
    y2016round1b::b::solve_all_cases();
    y2016round1b::c::solve_all_cases();
}

pub fn run_y2016_round_1c()
{
    y2016round1c::a::solve_all_cases();
    y2016round1c::b::solve_all_cases();
    y2016round1c::c::solve_all_cases();
}

pub fn run_y2016_round2()
{
    y2016round2::a::solve_all_cases();
    y2016round2::b::solve_all_cases();
    y2016round2::c::solve_all_cases();
    y2016round2::d::solve_all_cases();
}


pub fn run_y2016_round3()
{

}
fn main()
{
    init_log();

    y2016round3::a::solve_all_cases();
}
