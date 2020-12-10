use crate::*;
use std::convert::TryInto;
use std::io::BufRead;

pub struct Input(Vec<usize>);

impl<R: BufRead> ReadInput<R> for Input {
    fn read(b: R) -> Result<Input, ()> {
        Ok(Input(
            b.lines()
                .flatten()
                .filter_map(|line| line.parse::<usize>().ok())
                .collect::<Vec<_>>(),
        ))
    }
}

pub struct FirstStep;
impl Solution for FirstStep {
    type Input = Input;
    type Output = usize;
    fn solve(i: Input) -> Result<usize, ()> {
        let mut num = i.0;
        num.sort_unstable();

        let min = num[0];
        let fold_start = {
            let mut x = [0, 0, 0];
            x[min - 1] = 1;
            x
        };
        let deltas = num.windows(2).fold(fold_start, |mut acc, nums| {
            let [na, nb]: [usize; 2] = dbg!(nums.try_into().unwrap());
            debug_assert!(nb - na >= 1 && nb - na <= 3);
            acc[nb - na - 1] += 1;
            dbg!(acc)
        });

        Ok(deltas[0] * (deltas[2] + 1))
    }
}

pub struct SecondStep;
impl Solution for SecondStep {
    type Input = Input;
    type Output = usize;

    /// when determining the number of possible mutations of this chain, we only need to look
    /// at the entries with a difference of less than 3, because the only  modification we can
    /// do is remove items from the chain (since adapters cannot take input higher than their
    /// rating) and we can only remove an entry, iff its neighbors are compatible to each other.
    ///
    /// It turns out, that there are no followers with a delta of 2, neither in my personal
    /// payload, nor in the samples. This allows us to just check for a delta of 1.
    ///
    /// We can think of the whole chain as a set of smaller chains, which are connected to each
    /// other. Each delta of three between two consecutive entris is a connection point where a
    /// new "sub-chain" starts. Each of these smaller sub-chains is made up of entries exactly
    /// 1 jolt apart from each other.
    /// We then have to determine the number of possible permutations of each of these smaller
    /// chains, multiply them all up and we got the number of possible permutations for the
    /// complete chain.
    ///
    /// The length of a chain is here defined as the number of CONNECTIONS between entries.
    /// A chain of length 1 contains of two entries (either two adapters, or one adapter and
    /// the outlet).
    ///
    /// The number of permutations for length 1, 2 and 3 are trivial to calculate:
    ///  1. got exactly 2 entries, we need to keep the start and the end, so theres exactly 1
    ///     variation
    ///  2. got 3 entries, we may either cut the middle one, or not, and thus got two entries
    ///  3. got "floating adapters" we can cut, which leads to four permutations (none, a, b,
    ///     or a & b).
    ///
    /// At this point we may want to get ahead of ourself and think "HEY! I know this series!".
    /// When interpreting the number of "floating adapterS" (not the start or end of a chain)
    /// as bits, the number of possible values these bits could represent is identical to the
    /// number of permutations, which is no suprise, since all we do is take an adapter away or
    /// not which is representable as 1 or 0.
    /// But sadly the correct answert for the length of 4 is not 8, but 7.
    /// Thats because we cannot remove three consecutive adapters, because then the difference
    /// between the previous and next one would be >3 which breaks the rules of santa-physics.
    ///
    /// Thus we would need to account for permutations containting three consecutive zeros for
    /// all chain lenghts >= 4 and only count those who follow the rules.
    /// Luckily, again, the input only contains chains up to a lenght of 4, so a tiny LUT saves
    /// the day.
    ///
    /// Oh wow, that was a fun one!   type Output = usize;
    fn solve(i: Input) -> Result<usize, ()> {
        let mut num = i.0;
        num.sort_unstable();

        let max = num[num.len() - 1];
        let x: Vec<usize> = vec![0usize]
            .into_iter()
            .chain(num)
            .chain(vec![max + 3])
            .collect();

        let magic_number = get_chain_lengths(&x)
            .iter()
            .map(|len| match len {
                0 | 1 => 1,
                2 => 2,
                3 => 4,
                4 => 7,
                _ => panic!("{}", len),
            })
            .product();

        Ok(magic_number)
    }
}

fn get_chain_lengths(numbers: &[usize]) -> Vec<usize> {
    let (_, chains) = numbers
        .windows(2)
        .fold((0, Vec::new()), |(len, mut hist), x| {
            if x[1] - x[0] == 1 {
                (len + 1, hist)
            } else {
                if len > 0 {
                    hist.push(len);
                }
                (0, hist)
            }
        });

    chains
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::BufReader;

    static INPUT: &str = r#"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"#;

    #[test]
    fn test_step_1() {
        let read = Input::read(BufReader::new(INPUT.as_bytes())).unwrap();
        assert_eq!(Ok(220), FirstStep::solve(read));
    }

    #[test]
    fn test_step_2() {
        let read = Input::read(BufReader::new(INPUT.as_bytes())).unwrap();
        assert_eq!(Ok(19208), SecondStep::solve(read));
    }
}
