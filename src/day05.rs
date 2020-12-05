use crate::*;
use std::io::BufRead;

pub struct Input(Vec<usize>);

impl<R: BufRead> ReadInput<R> for Input {
    fn read(b: R) -> Result<Input, ()> {
        Ok(Input(
            b.lines()
                .filter_map(|maybe_line| maybe_line.ok())
                .filter_map(|line| to_seat_id(&line).ok())
                .collect(),
        ))
    }
}

fn to_seat_id(inp: &str) -> Result<usize, ()> {
    let mut num = 0;
    for bit in inp.chars().map(|chr| chr == 'B' || chr == 'R') {
        num *= 2;
        if bit {
            num += 1;
        }
    }

    Ok(num)
}

pub struct FirstStep;
impl Solution for FirstStep {
    type Input = Input;
    type Output = usize;
    fn solve(i: Input) -> Result<usize, ()> {
        Ok(*i.0.iter().max().unwrap())
    }
}

pub struct SecondStep;
impl Solution for SecondStep {
    type Input = Input;
    type Output = usize;
    fn solve(i: Input) -> Result<usize, ()> {
        let mut v = i.0;
        v.sort_unstable();
        let found = v.windows(2).find(|w| w[1] == w[0] + 2);

        Ok(found.unwrap()[0] + 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::BufReader;

    static INPUT: &str = r#"BFFFBBFRRR
FFFBBBFRLR
FFFBBBFRRR
BBFFBBFRLL"#;

    #[test]
    fn test_step_1() {
        let read = Input::read(BufReader::new(INPUT.as_bytes())).unwrap();
        assert_eq!(Ok(820), FirstStep::solve(read));
    }

    #[test]
    fn test_step_2() {
        let read = Input::read(BufReader::new(INPUT.as_bytes())).unwrap();
        assert_eq!(Ok(118), SecondStep::solve(read));
    }
}
