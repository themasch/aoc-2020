use crate::*;
use std::convert::TryInto;
use std::io::BufRead;
use std::ops::RangeInclusive;

pub struct Input(Vec<PasswordRecord>);
pub struct PasswordRecord {
    range: RangeInclusive<usize>,
    character: char,
    password: String,
}

impl<R: BufRead> ReadInput<R> for Input {
    fn read(b: R) -> Result<Input, ()> {
        let rows = b
            .lines()
            .filter_map(|l| l.ok())
            .map(|line| {
                let [range, split, password]: [&str; 3] =
                    line.split(' ').collect::<Vec<_>>().try_into().unwrap();
                let [from, to]: [usize; 2] = range
                    .split('-')
                    .map(|i| i.parse().unwrap())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();

                let chr = split.chars().next().unwrap();
                debug_assert_eq!(Some(':'), split.chars().nth(1));

                PasswordRecord {
                    range: from..=to,
                    character: chr,
                    password: String::from(password),
                }
            })
            .collect();

        Ok(Input(rows))
    }
}

pub struct FirstStep;
impl Solution for FirstStep {
    type Input = Input;
    type Output = usize;
    fn solve(i: Input) -> Result<usize, ()> {
        let count =
            i.0.iter()
                .filter(|record| {
                    let matches = record
                        .password
                        .chars()
                        .filter(|&chr| record.character == chr)
                        .count();
                    record.range.contains(&matches)
                })
                .count();

        Ok(count)
    }
}

pub struct SecondStep;
impl Solution for SecondStep {
    type Input = Input;
    type Output = usize;
    fn solve(i: Input) -> Result<usize, ()> {
        let count =
            i.0.iter()
                .filter(|record| {
                    let chars: Vec<char> = record.password.chars().collect();
                    let (start, end) = record.range.clone().into_inner();
                    let places: (char, char) = (chars[start - 1], chars[end - 1]);

                    places.0 != places.1
                        && (places.0 == record.character || places.1 == record.character)
                })
                .count();

        Ok(count)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn test_step_1() {
        let input = r#"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"#;
        let read = Input::read(BufReader::new(input.as_bytes())).unwrap();
        assert_eq!(Ok(2), FirstStep::solve(read));
    }

    #[test]
    fn test_step_2() {
        let input = r#"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"#;
        let read = Input::read(BufReader::new(input.as_bytes())).unwrap();
        assert_eq!(Ok(1), SecondStep::solve(read));
    }
}
