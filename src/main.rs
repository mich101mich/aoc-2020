#![allow(unused_imports)]

#[macro_use]
extern crate scan_fmt;

#[macro_use]
mod utils;
mod neighbors;
mod days {
    pub mod day_13;
}
use days::day_13;

fn main() {
    day_13::run();
}
