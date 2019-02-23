//#![allow(non_snake_case)]

mod y2017qual;
mod y2017round1a;
mod y2017round1b;
mod y2017round1c;
mod y2017round2;
mod y2017round3;
mod y2017round4;

extern crate codejam;

/*#[macro_use]
extern crate itertools;*/

#[macro_use]
extern crate log;

#[macro_use]
extern crate try_opt;

use codejam::util::log::init_log;

pub fn run_y2017_round_qual()
{
    y2017qual::a::solve_all_cases();
    y2017qual::b::solve_all_cases();
    y2017qual::c::solve_all_cases();
}

pub fn run_y2017_round_1a()
{
    y2017round1a::a::solve_all_cases();
    y2017round1a::b::solve_all_cases();
    y2017round1a::c::solve_all_cases();
}

pub fn run_y2017_round_1b()
{
    y2017round1b::a::solve_all_cases();
    y2017round1b::b::solve_all_cases();
    y2017round1b::c::solve_all_cases();
}

pub fn run_y2017_round_1c()
{
    y2017round1c::a::solve_all_cases();
    y2017round1c::b::solve_all_cases();
    y2017round1c::c::solve_all_cases();
}

pub fn run_y2017_round2()
{
    y2017round2::a::solve_all_cases();
    y2017round2::b::solve_all_cases();
    y2017round2::c::solve_all_cases();
    y2017round2::d::solve_all_cases();
}

pub fn run_y2017_round3()
{
    y2017round3::a::solve_all_cases();
    y2017round3::b::solve_all_cases();
    y2017round3::c::solve_all_cases();
    y2017round3::d::solve_all_cases();
}

pub fn run_y2017_round4()
{
    y2017round4::a::solve_all_cases();
    y2017round4::b::solve_all_cases();
    y2017round4::c::solve_all_cases();
    y2017round4::d::solve_all_cases();
    y2017round4::e::solve_all_cases();
    y2017round4::f::solve_all_cases();
}
fn main()
{
    init_log();

    run_y2017_round1a();
    run_y2017_round1b();
    run_y2017_round2();
    //run_y2017_round4();
}
