#![allow(non_snake_case)]
mod y2016qual;
mod y2016round1A;
mod y2016round1B;
mod y2016round1C;
mod y2016round2;

extern crate codejam;

#[macro_use]
extern crate itertools;

#[macro_use]
extern crate log;

use codejam::util::log::init_log;

pub fn run_y2016_roundQual()
{
    y2016qual::a::solve_all_cases();
    y2016qual::b::solve_all_cases();
    y2016qual::c::solve_all_cases();
    y2016qual::d::solve_all_cases();
}

pub fn run_y2016_round1A()
{
    y2016round1A::a::solve_all_cases();
    y2016round1A::b::solve_all_cases();
    y2016round1A::c::solve_all_cases();
}

pub fn run_y2016_round1B()
{
    y2016round1B::a::solve_all_cases();
    y2016round1B::b::solve_all_cases();
    y2016round1B::c::solve_all_cases();
}

pub fn run_y2016_round1C()
{
    y2016round1C::a::solve_all_cases();
    y2016round1C::b::solve_all_cases();    
    y2016round1C::c::solve_all_cases();
}

pub fn run_y2016_round2()
{
    y2016round2::a::solve_all_cases();
    y2016round2::b::solve_all_cases();
}

fn main()
{
    init_log();

 y2016round1A::b::solve_all_cases();

}
