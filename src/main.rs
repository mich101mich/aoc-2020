#![allow(unused_imports)]

#[macro_use]
extern crate scan_fmt;

#[macro_use]
mod utils;
mod days {
    pub mod day_08;
}
use days::day_08;

fn main() {
    day_08::run();
}
