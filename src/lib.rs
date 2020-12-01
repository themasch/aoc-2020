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

pub fn run_solution<S: Solution, R: Read + Sized>(from: R) -> S::Output
where
    S::Input: ReadInput<R>,
{
    let input = S::Input::read(from).unwrap();
    S::solve(input).unwrap()
}

pub mod day1;

