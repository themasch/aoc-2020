use crate::*;
use std::io::BufRead;

#[derive(Debug)]
pub struct Input(Vec<Passport>);
#[derive(Debug)]
pub struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}

use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::iter::FromIterator;
impl Passport {
    fn try_create_from(input: Vec<(&str, &str)>) -> Option<Passport> {
        let map: HashMap<&str, &str, RandomState> = HashMap::from_iter(input);

        Some(Passport {
            byr: map.get("byr").map(|&x| String::from(x))?,
            iyr: map.get("iyr").map(|&x| String::from(x))?,
            eyr: map.get("eyr").map(|&x| String::from(x))?,
            hgt: map.get("hgt").map(|&x| String::from(x))?,
            hcl: map.get("hcl").map(|&x| String::from(x))?,
            ecl: map.get("ecl").map(|&x| String::from(x))?,
            pid: map.get("pid").map(|&x| String::from(x))?,
            cid: map.get("cid").map(|&x| String::from(x)),
        })
    }
}

impl<R: BufRead> ReadInput<R> for Input {
    fn read(mut b: R) -> Result<Input, ()> {
        let mut content = String::new();
        b.read_to_string(&mut content).unwrap();
        let vec = content
            .replace("\r\n", "\n")
            .split("\n\n")
            .filter_map(|input| read_passport(input))
            .collect();

        Ok(Input(vec))
    }
}

fn read_passport(input: &str) -> Option<Passport> {
    let kv_pairs = input
        .split_whitespace()
        .filter_map(|pair| {
            if pair.contains(':') {
                let key_value: Vec<&str> = pair.split(':').collect();
                Some((key_value[0], key_value[1]))
            } else {
                None
            }
        })
        .collect();

    Passport::try_create_from(kv_pairs)
}

pub struct FirstStep;
impl Solution for FirstStep {
    type Input = Input;
    type Output = usize;
    fn solve(i: Input) -> Result<usize, ()> {
        Ok(i.0.len())
    }
}

enum HeightUnit {
    Centimeter,
    Inch,
}
struct Height {
    amount: usize,
    unit: HeightUnit,
}

impl Height {
    fn as_cm(&self) -> f32 {
        match self.unit {
            HeightUnit::Centimeter => self.amount as f32,
            HeightUnit::Inch => self.amount as f32 * 2.54,
        }
    }
}

use std::str::FromStr;
impl FromStr for Height {
    type Err = ();
    fn from_str(input: &str) -> Result<Height, ()> {
        if input.ends_with("in") {
            Ok(Height {
                amount: input[0..input.len() - 2].parse().map_err(|_| ())?,
                unit: HeightUnit::Inch,
            })
        } else if input.ends_with("cm") {
            Ok(Height {
                amount: input[0..input.len() - 2].parse().map_err(|_| ())?,
                unit: HeightUnit::Centimeter,
            })
        } else {
            Err(())
        }
    }
}

pub struct SecondStep;
impl Solution for SecondStep {
    type Input = Input;
    type Output = usize;
    fn solve(i: Input) -> Result<usize, ()> {
        let output =
            i.0.iter()
                .filter(|pp| {
                    pp.byr
                        .parse::<u16>()
                        .ok()
                        .map(|value| value >= 1920 && value <= 2002)
                        .unwrap_or(false)
                })
                .filter(|pp| {
                    pp.iyr
                        .parse::<u16>()
                        .ok()
                        .map(|value| value >= 2010 && value <= 2020)
                        .unwrap_or(false)
                })
                .filter(|pp| {
                    pp.eyr
                        .parse::<u16>()
                        .ok()
                        .map(|value| value >= 2020 && value <= 2030)
                        .unwrap_or(false)
                })
                .filter(|pp| {
                    pp.hgt
                        .parse::<Height>()
                        .ok()
                        .map(|value| value.as_cm() >= 150.0 && value.as_cm() <= 193.0)
                        .unwrap_or(false)
                })
                .filter(|pp| {
                    pp.hcl.as_str().starts_with('#')
                        && pp.hcl[1..]
                            .chars()
                            .find(|&c| (c < '0' || c > '9') && (c < 'a' || c > 'f'))
                            .is_none()
                })
                .filter(|pp| {
                    let allowed_colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
                    allowed_colors.contains(&pp.ecl.as_str())
                })
                .filter(|pp| {
                    pp.pid.len() == 9 && pp.pid.chars().find(|&c| c < '0' || c > '9').is_none()
                })
                .count();

        Ok(output)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::BufReader;

    static INPUT: &str = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"#;

    #[test]
    fn test_read_passports() {
        Input::read(BufReader::new(INPUT.as_bytes())).unwrap();
    }

    #[test]
    fn test_step_1() {
        let read = Input::read(BufReader::new(INPUT.as_bytes())).unwrap();
        assert_eq!(Ok(2), FirstStep::solve(read));
    }

    #[test]
    fn test_step_2_all_invalid() {
        let local_input = r#"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"#;
        let read = Input::read(BufReader::new(local_input.as_bytes())).unwrap();
        assert_eq!(Ok(0), SecondStep::solve(read));
    }

    #[test]
    fn test_step_2_all_valid() {
        let local_input = r#"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"#;
        let read = Input::read(BufReader::new(local_input.as_bytes())).unwrap();
        assert_eq!(Ok(4), SecondStep::solve(read));
    }
}
