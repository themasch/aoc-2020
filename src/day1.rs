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

        for a in list.iter().rev() {
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
        let len = list.len();

        for a in 0..len {
            for b in (a + 1)..len {
                for c in (b + 1)..len {
                    let (va, vb, vc) = (list[a], list[b], list[c]);
                    if va + vb + vc == 2020 {
                        return Ok(va * vb * vc);
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
