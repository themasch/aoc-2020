use chrono::prelude::*;
use std::fs::File;
use std::io::{BufReader, Read};

pub trait Solution {
    type Input;
    type Output;

    fn solve(input: Self::Input) -> Result<Self::Output, ()>;
}

pub trait ReadInput<R>
where
    R: Read,
    Self: Sized,
{
    fn read(r: R) -> Result<Self, ()>;
}

pub fn run_solution<S: Solution, R: Read + Sized>(from: R) -> S::Output
where
    S::Input: ReadInput<R>,
{
    let input = S::Input::read(from).unwrap();
    S::solve(input).unwrap()
}

mod day1;


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
    let today = if let Some(param) = std::env::args().skip(1).next() {
        param.parse().unwrap()
    } else {
        Local::now().day()
    };

    daymap!(today, {
        1 => day1
    });
}
