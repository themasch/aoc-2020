use crate::*;
use std::io::BufRead;

#[derive(Debug, Clone)]
pub struct Input(Vec<usize>, usize);

impl<R: BufRead> ReadInput<R> for Input {
    fn read(b: R) -> Result<Input, ()> {
        Ok(Input(
            b.lines()
                .flatten()
                .filter_map(|line| line.parse::<usize>().ok())
                .collect::<Vec<_>>(),
            25,
        ))
    }
}

pub struct FirstStep;
impl Solution for FirstStep {
    type Input = Input;
    type Output = usize;
    fn solve(i: Input) -> Result<usize, ()> {
        let Input(numbers, preamble) = i;
        for (offset, num) in numbers[preamble..].iter().enumerate() {
            let pre = &numbers[offset..offset + preamble];
            let found = find_match(*num, pre);

            if !found {
                return Ok(*num);
            }
        }

        Err(())
    }
}

fn find_match(num: usize, numbers: &[usize]) -> bool {
    for a in numbers.iter() {
        for b in numbers.iter().rev() {
            if a != b && a + b == num {
                return true;
            }
        }
    }

    false
}

pub struct SecondStep;
impl Solution for SecondStep {
    type Input = Input;
    type Output = usize;
    fn solve(i: Input) -> Result<usize, ()> {
        let searched_number = FirstStep::solve(i.clone()).unwrap();
        for idx in 0..i.0.len() {
            let slice_size = find_matching_slice(searched_number, &i.0[idx..]);
            if let Some(size) = slice_size {
                let slice = &i.0[idx..idx + size];
                let min = slice.iter().min().unwrap();
                let max = slice.iter().max().unwrap();
                return Ok(min + max);
            }
        }
        unimplemented!()
    }
}

fn find_matching_slice(target: usize, numbers: &[usize]) -> Option<usize> {
    let mut acc = 0;
    for (idx, num) in numbers.iter().enumerate() {
        acc += num;
        if acc == target {
            return Some(idx);
        }

        if acc > target {
            return None;
        }
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::BufReader;

    static INPUT: &str = r#"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"#;

    #[test]
    fn test_step_1() {
        let Input(num, _) = Input::read(BufReader::new(INPUT.as_bytes())).unwrap();
        assert_eq!(Ok(127), FirstStep::solve(Input(num, 5)));
    }

    #[test]
    fn test_step_2() {
        let Input(num, _) = Input::read(BufReader::new(INPUT.as_bytes())).unwrap();
        assert_eq!(Ok(62), SecondStep::solve(Input(num, 5)));
    }
}
