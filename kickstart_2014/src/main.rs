mod y2014_round_a;

extern crate codejam;

#[macro_use]
extern crate log;

use codejam::util::log::init_log;

pub fn run_y2014_round_a()
{
    y2014_round_a::a::solve_all_cases();
}

fn main()
{
    init_log();
    y2014_round_a::a::solve_all_cases();
}
