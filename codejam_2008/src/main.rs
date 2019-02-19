//#![allow(non_snake_case)]

mod y2008beta;
mod y2008practice;
mod y2008qual;
mod y2008round1a;
mod y2008round1b;

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

    y2008practice::a::solve_all_cases();
    y2008practice::b::solve_all_cases();
    y2008practice::c::solve_all_cases();
    y2008practice::d::solve_all_cases();
}

pub fn run_y2008_round_beta()
{
    y2008beta::a::solve_all_cases();
    y2008beta::b::solve_all_cases();
    y2008beta::c::solve_all_cases();
    y2008beta::d::solve_all_cases();
}

pub fn run_y2008_practice_contest()
{
    //https://code.google.com/codejam/contest/32004/dashboard#s=p2

    //TODO
}

pub fn run_y2008_round_1a()
{
    y2008round1a::a::solve_all_cases();
    y2008round1a::b::solve_all_cases();
    y2008round1a::c::solve_all_cases();
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

    y2008round1b::a::solve_all_cases()
    /*   run_y2008_round_qual();
    run_y2008_round_practice();
    run_y2008_round_beta();*/
}
