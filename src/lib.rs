use std::io::Read;

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

pub fn run_solution<S: Solution, R: Read + Sized>(from: R) -> Result<S::Output, ()>
where
    S::Input: ReadInput<R>,
{
    S::solve(ReadInput::read(from).unwrap())
}

pub mod day02;
pub mod day1;
