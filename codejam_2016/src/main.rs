

#![allow(non_snake_case)]
mod y2016qual;
mod y2016round1A;

extern crate codejam;


use codejam::util::log::init_log;

pub fn run_y2016qual()
{
  y2016qual::a::solve_all_cases();
  y2016qual::b::solve_all_cases();
  y2016qual::c::solve_all_cases();
  y2016qual::d::solve_all_cases();
}

fn main()
{
    init_log();

/*
 
    */

    //y2016round1A::a::solve_all_cases();
    y2016round1A::b::solve_all_cases();
}
