mod y2014_round_a;

extern crate codejam;

#[macro_use]
extern crate log;

use codejam::util::log::init_log;

pub fn run_y2008_round_qual()
{
    y2008qual::a::solve_all_cases();
    y2008qual::b::solve_all_cases();
    y2008qual::c::solve_all_cases();
}

fn main() {
    println!("Hello, world!");
}

