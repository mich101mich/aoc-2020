#![allow(unused_imports)]

#[macro_use]
extern crate scan_fmt;

#[macro_use]
mod utils;
mod days {
    pub mod day_10;
}
use days::day_10;

fn main() {
    day_10::run();
}
