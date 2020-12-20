DAY=$1
echo "use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!(\"../input/$DAY.txt\");
    // let input = "";
    
    let parsed = input
        //.lines()
        //.map(parse)
        //.map(|l| scanf!(l, \"{}\", i64))
        //.to_vec()
        //.sum::<i64>()
        ;
    
    //pv!(parsed);
    
}" > src/days/day_$DAY.rs

echo "#![allow(unused_imports)]

#[macro_use]
extern crate scan_fmt;

#[macro_use]
mod utils;
mod neighbors;
mod days {
    pub mod day_$DAY;
}
use days::day_$DAY;

fn main() {
    day_$DAY::run();
}" > src/main.rs

touch src/input/$DAY.txt

if [ -f cmd.exe ]; then
    cmd.exe /c code src/days/day_$DAY.rs src/input/$DAY.txt
else
    code src/days/day_$DAY.rs src/input/$DAY.txt
fi
