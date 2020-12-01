use std::io::BufRead;

#[derive(Debug, Clone)]
pub struct Input(Vec<usize>);
impl<R: BufRead> crate::ReadInput<R> for Input {
    fn read(r: R) -> Result<Input, ()> {
        let vec = r
            .lines()
            .map(|line| line.unwrap().parse().unwrap())
            .collect();
        Ok(Input(vec))
    }
}

pub struct FirstStep;
impl crate::Solution for FirstStep {
    type Input = Input;
    type Output = usize;

    fn solve(input: Self::Input) -> Result<Self::Output, ()> {
        let list = input.0;

        for a in list.clone().iter().rev() {
            for b in &list {
                if a + b == 2020 {
                    return Ok(a * b);
                }
            }
        }

        Err(())
    }
}

pub struct SecondStep;
impl crate::Solution for SecondStep {
    type Input = Input;
    type Output = usize;

    fn solve(input: Self::Input) -> Result<Self::Output, ()> {
        let list = input.0;

        for a in &list { 
            for b in &list {
                for c in &list {
                    if a + b + c == 2020 {
                        return Ok(a * b * c);
                    }
                }
            }
        }

        Err(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{ReadInput, Solution};

    #[test]
    fn test_first_step() {
        let input = r#"1721
979
366
299
675
1456"#;
        assert_eq!(
            Ok(514579),
            FirstStep::solve(Input::read(input.as_bytes()).unwrap())
        );
    }

    #[test]
    fn test_second_step() {
        let input = r#"1721
979
366
299
675
1456"#;
        assert_eq!(
            Ok(241861950),
            SecondStep::solve(Input::read(input.as_bytes()).unwrap())
        );
    }
}
