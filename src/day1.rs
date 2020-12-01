use std::io::BufRead;

pub struct Day01Input(Vec<usize>);
impl<R: BufRead> crate::ReadInput<R> for Day01Input {
    fn read(r: R) -> Result<Day01Input, ()> {
        let vec = r
            .lines()
            .map(|line| line.unwrap().parse().unwrap())
            .collect();
        Ok(Day01Input(vec))
    }
}

pub struct FirstStep;
impl crate::Solution for FirstStep {
    type Input = Day01Input;
    type Output = usize;

    fn solve(input: Self::Input) -> Result<Self::Output, ()> {
        let list = input.0;
        let blist = {
            let mut t = list.clone();
            t.reverse();
            t
        };

        for a in list {
            for b in &blist {
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
    type Input = Day01Input;
    type Output = usize;

    fn solve(input: Self::Input) -> Result<Self::Output, ()> {
        let list = input.0;
        let blist = {
            let mut t = list.clone();
            t.reverse();
            t
        };

        for a in &list {
            for b in &blist {
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
            FirstStep::solve(Day01Input::read(input.as_bytes()).unwrap())
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
            SecondStep::solve(Day01Input::read(input.as_bytes()).unwrap())
        );
    }
}
