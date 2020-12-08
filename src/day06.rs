use crate::*;
use std::collections::HashSet;
use std::io::BufRead;
use std::iter::FromIterator;

pub struct Input(Vec<Vec<HashSet<char>>>);

impl<R: BufRead> ReadInput<R> for Input {
    fn read(mut b: R) -> Result<Input, ()> {
        let mut buffer = String::new();
        b.read_to_string(&mut buffer).unwrap();
        let vec = buffer
            .replace("\r\n", "\n")
            .split("\n\n")
            .map(|x| {
                x.lines()
                    .map(|x| HashSet::from_iter(x.chars().filter(|&c| c >= 'a' && c <= 'z')))
                    .collect()
            })
            .collect();

        Ok(Input(vec))
    }
}

pub struct FirstStep;
impl Solution for FirstStep {
    type Input = Input;
    type Output = usize;
    fn solve(i: Input) -> Result<usize, ()> {
        Ok(i.0
            .iter()
            .map(|sets| {
                sets.iter()
                    .fold(HashSet::new(), |acc, set| acc.union(set).cloned().collect())
                    .len()
            })
            .sum())
    }
}

pub struct SecondStep;
impl Solution for SecondStep {
    type Input = Input;
    type Output = usize;
    fn solve(i: Input) -> Result<usize, ()> {
        Ok(i.0
            .iter()
            .map(|sets| {
                sets.iter()
                    .fold(None, |acc: Option<HashSet<char>>, set| {
                        if let Some(acc) = acc {
                            Some(acc.intersection(set).cloned().collect())
                        } else {
                            Some(set.clone())
                        }
                    })
                    .unwrap()
                    .len()
            })
            .sum())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::BufReader;

    static INPUT: &str = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;

    #[test]
    fn test_step_1() {
        let read = Input::read(BufReader::new(INPUT.as_bytes())).unwrap();
        assert_eq!(Ok(11), FirstStep::solve(read));
    }

    #[test]
    fn test_step_2() {
        let read = Input::read(BufReader::new(INPUT.as_bytes())).unwrap();
        assert_eq!(Ok(6), SecondStep::solve(read));
    }
}
