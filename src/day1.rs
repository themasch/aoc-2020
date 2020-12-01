use std::collections::HashSet;
use std::io::BufRead;

#[derive(Debug, Clone)]
pub struct Input(HashSet<usize>);
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

        for a in &list {
            let delta = 2020 - a;
            if list.contains(&delta) {
                return Ok(a * delta);
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
            for delta in list.iter().filter_map(|b| 2020usize.checked_sub(a + b)) {
                if list.contains(&delta) {
                    return Ok(a * delta * (2020 - delta - a));
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
            FirstStep::solve(<FirstStep as Solution>::Input::read(input.as_bytes()).unwrap())
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
            SecondStep::solve(<SecondStep as Solution>::Input::read(input.as_bytes()).unwrap())
        );
    }
}
