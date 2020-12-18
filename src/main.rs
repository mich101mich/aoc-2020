#![allow(unused_imports)]

#[macro_use]
extern crate scan_fmt;

#[macro_use]
mod utils;
mod neighbors;
mod days {
    pub mod day_18;
}
use days::day_18;

fn main() {
    day_18::run();
}
