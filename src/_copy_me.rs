use crate::*;
use std::convert::TryInto;
use std::io::BufRead;
use std::ops::RangeInclusive;

pub struct Input(_);

impl<R: BufRead> ReadInput<R> for Input {
    fn read(b: R) -> Result<Input, ()> {
        unimplemented!()
    }
}

pub struct FirstStep;
impl Solution for FirstStep {
    type Input = Input;
    type Output = usize;
    fn solve(i: Input) -> Result<usize, ()> {
        unimplemented!()
    }
}

pub struct SecondStep;
impl Solution for SecondStep {
    type Input = Input;
    type Output = usize;
    fn solve(i: Input) -> Result<usize, ()> {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn test_step_1() {

        let read = Input::read(BufReader::new(input.as_bytes())).unwrap();
        assert_eq!(Ok(2), FirstStep::solve(read));
    }

    #[test]
    fn test_step_2() {

        let read = Input::read(BufReader::new(input.as_bytes())).unwrap();
        assert_eq!(Ok(1), SecondStep::solve(read));
    }
}
