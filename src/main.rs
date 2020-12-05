use aoc_2020::*;
use chrono::prelude::*;
use std::fs::File;
use std::io::BufReader;

macro_rules! daymap {
    ($key:ident, { $($idx:expr => $mod:ident),+ }) => {
        match $key {
            $($idx => {
                println!("solutions for day {}", $idx);
                let filename = format!("inputs/day{:02}_input.txt", $idx);
                let read = BufReader::new(File::open(&filename).unwrap());
                println!("1st step: {:?}", run_solution::<$mod::FirstStep, _>(read));
                let read = BufReader::new(File::open(&filename).unwrap());
                println!("2nd step: {:?}", run_solution::<$mod::SecondStep, _>(read));
            }),+
            _ => panic!("no such day (yet?): {}", $key)
        }
    };
}

fn main() {
    let today = if let Some(param) = std::env::args().nth(1) {
        param.parse().unwrap()
    } else {
        Local::now().day()
    };

    daymap!(today, {
        1 => day1,
        2 => day02,
        3 => day03,
        4 => day04
    });
}
