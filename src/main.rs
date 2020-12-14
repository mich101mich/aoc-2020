#![allow(unused_imports)]

#[macro_use]
extern crate scan_fmt;

#[macro_use]
mod utils;
mod neighbors;
mod days {
    pub mod day_14;
}
use days::day_14;

fn main() {
    day_14::run();
}
