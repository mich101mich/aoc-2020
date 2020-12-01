#![allow(unused_imports)]

#[macro_use]
extern crate scan_fmt;

#[macro_use]
mod utils;
mod days {
	pub mod day_1;
}
use days::day_1;

fn main() {
    day_1::run();
}
