#![allow(unused_imports)]

#[macro_use]
extern crate scan_fmt;

#[macro_use]
mod utils;
mod neighbors;
mod days {
    pub mod day_11;
}
use days::day_11;

fn main() {
    day_11::run();
}
