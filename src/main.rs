#![allow(unused_imports)]

#[macro_use]
extern crate scan_fmt;

#[macro_use]
mod utils;
mod neighbors;
mod days {
    pub mod day_19;
}
use days::day_19;

fn main() {
    day_19::run();
}
