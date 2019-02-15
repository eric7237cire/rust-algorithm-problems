//#![allow(non_snake_case)]
mod y2008qual;
mod y2008practice;

extern crate codejam;

/*#[macro_use]
extern crate itertools;*/

#[macro_use]
extern crate log;

use codejam::util::log::init_log;

pub fn run_y2008_round_qual()
{
    y2008qual::a::solve_all_cases();
    y2008qual::b::solve_all_cases();
    y2008qual::c::solve_all_cases();
}

pub fn run_y2008_round_practice()
{
    //https://code.google.com/codejam/contest/32003/dashboard
}

pub fn run_y2008_round_1a()
{

}

pub fn run_y2008_round_1b()
{

}

pub fn run_y2008_round_1c()
{

}

pub fn run_y2008_round2()
{

}

fn main()
{
    init_log();


    y2008practice::a::solve_all_cases();
}