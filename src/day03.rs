use crate::*;
use std::io::BufRead;

pub struct Input {
    width: usize,
    height: usize,
    fields: Vec<char>,
}

impl<R: BufRead> ReadInput<R> for Input {
    fn read(b: R) -> Result<Input, ()> {
        let lines: Vec<_> = b.lines().collect::<Result<Vec<_>, _>>().unwrap();
        let width = lines[0].len();
        let height = lines.len();
        let fields = lines.iter().flat_map(|c| c.chars()).collect();
        Ok(Input {
            width,
            fields,
            height,
        })
    }
}

impl Input {
    fn get_char(&self, x: usize, y: usize) -> char {
        self.fields[y * self.width + (x % self.width)]
    }
}

fn get_slope_count(field: &Input, dx: usize, dy: usize) -> usize {
    (0..(field.height as f64 / dy as f64).ceil() as usize)
        .filter(|&row| field.get_char(row * dx, row * dy) == '#')
        .count()
}

pub struct FirstStep;
impl Solution for FirstStep {
    type Input = Input;
    type Output = usize;
    fn solve(i: Input) -> Result<usize, ()> {
        Ok(get_slope_count(&i, 3, 1))
    }
}

pub struct SecondStep;
impl Solution for SecondStep {
    type Input = Input;
    type Output = usize;
    fn solve(i: Input) -> Result<usize, ()> {
        Ok(get_slope_count(&i, 1, 1)
            * get_slope_count(&i, 3, 1)
            * get_slope_count(&i, 5, 1)
            * get_slope_count(&i, 7, 1)
            * get_slope_count(&i, 1, 2))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn pos_access_in_input() {
        let i = Input {
            width: 3,
            height: 2,
            fields: vec!['a', 'b', 'c', 'd', 'e', 'f'],
        };

        assert_eq!('a', i.get_char(0, 0));
        assert_eq!('a', i.get_char(3, 0));
        assert_eq!('a', i.get_char(6, 0));
        assert_eq!('d', i.get_char(6, 1));
        assert_eq!('f', i.get_char(2, 1));
    }

    #[test]
    fn check_slope_count() {
        let i = Input {
            width: 1,
            height: 4,
            fields: vec!['.', '#', '.', '#'],
        };

        assert_eq!(0, get_slope_count(&i, 1, 2));
    }

    #[test]
    fn test_down_slope() {
        let input = r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.##..##..#.
..#.##.....
.#.#.#....#
.#........#
#.###..#...
#...##....#
.#..##..#.#"#;
        let i = Input::read(BufReader::new(input.as_bytes())).unwrap();
        assert_eq!(5, get_slope_count(&i, 1, 2));
    }

    #[test]
    fn test_slope_counts() {
        let input = r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"#;
        let i = Input::read(BufReader::new(input.as_bytes())).unwrap();
        assert_eq!(2, get_slope_count(&i, 1, 1));
        assert_eq!(7, get_slope_count(&i, 3, 1));
        assert_eq!(3, get_slope_count(&i, 5, 1));
        assert_eq!(4, get_slope_count(&i, 7, 1));
        assert_eq!(2, get_slope_count(&i, 1, 2));
    }

    #[test]
    fn test_step_1() {
        let input = r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"#;
        let read = Input::read(BufReader::new(input.as_bytes())).unwrap();
        assert_eq!(Ok(7), FirstStep::solve(read));
    }

    #[test]
    fn test_step_2() {
        let input = r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"#;

        let read = Input::read(BufReader::new(input.as_bytes())).unwrap();
        assert_eq!(Ok(336), SecondStep::solve(read));
    }
}
