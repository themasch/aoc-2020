use crate::*;
use std::collections::HashMap;
use std::io::BufRead;

type BagMap = HashMap<String, HashMap<String, usize>>;
#[derive(Debug)]
pub struct Input(BagMap);

impl<R: BufRead> ReadInput<R> for Input {
    fn read(b: R) -> Result<Input, ()> {
        let v: BagMap = b
            .lines()
            .flatten()
            // we do not care about bags that contain nothing
            .filter(|line| !line.contains("no other bags."))
            .flat_map(|line| parse_bag_line(&line))
            .collect();

        Ok(Input(v))
    }
}

use std::convert::TryInto;

// drab bronze bags contain 3 wavy silver bags, 4 light turquoise bags, 1 vibrant lavender bag, 5 light magenta bags.
// {color} bags contain (({num} {color} bags, )*{num} {color} bags.)|(no other bags.)
fn parse_bag_line(line: &str) -> BagMap {
    static SPLITTER: &str = " bags contain ";
    let first_pos = line.find(SPLITTER).unwrap();
    let color = String::from(&line[0..first_pos]);
    let contains = line[first_pos + SPLITTER.len()..]
        .trim_end_matches('.')
        .split(',')
        .map(|l| l.trim())
        .collect::<Vec<&str>>();

    let submap = contains
        .iter()
        .map(|contain| {
            let [count, ccolor]: [&str; 2] = contain
                .splitn(2, ' ')
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            let ccolor = if ccolor.ends_with("bags") {
                &ccolor[0..ccolor.len() - 5]
            } else {
                &ccolor[0..ccolor.len() - 4]
            };

            (String::from(ccolor), count.parse().unwrap())
        })
        .collect::<HashMap<String, usize>>();

    let mut map = HashMap::new();
    map.insert(color, submap);

    map
}

pub struct FirstStep;
impl Solution for FirstStep {
    type Input = Input;
    type Output = usize;
    fn solve(i: Input) -> Result<usize, ()> {
        /*        let mut flip_map = HashMap::new();
        for (s_color, submap) in i.0 {
            for (t_color, count) in submap {
                if !flip_map.contains_key(&t_color) {
                    flip_map.insert(t_color.clone(), HashMap::new());
                }

                flip_map.get_mut(&t_color).unwrap().insert(s_color.clone(), count);
            }
        }

        let mut valid = HashSet::new();
        let mut visited = HashSet::new();

        visited.insert(String::from("shiny gold"));
        for color in flip_map["shiny gold"].keys() {
           valid.insert(color.clone());
        }

        for t_color in valid.difference(&visited) {
            for color in flip_map[t_color.as_str()].keys() {
                valid.insert(color.clone());
            }
        }

        dbg!(flip_map);*/

        let mut checked = HashMap::new();
        for (color, subbag) in &i.0 {
            if !checked.contains_key(color) {
                let res = check_bag(subbag, &i.0, &mut checked);
                checked.insert(String::from(color), res);
            }
        }

        Ok(checked.iter().filter(|(_, &b)| b).count())
    }
}

fn check_bag(
    subbag: &HashMap<String, usize>,
    all_bags: &BagMap,
    mut checked: &mut HashMap<String, bool>,
) -> bool {
    if subbag.contains_key("shiny gold") {
        return true;
    }

    for color in subbag.keys() {
        if checked.contains_key(color) {
            if checked[color] {
                return true;
            } else {
                continue;
            }
        }
        if all_bags.contains_key(color) && check_bag(&all_bags[color], all_bags, &mut checked) {
            checked.insert(String::from(color), true);
            return true;
        } else {
            if checked.contains_key(color) {
                debug_assert_eq!(false, checked[color]);
            }
            checked.insert(String::from(color), false);
        }
    }

    false
}

pub struct SecondStep;
impl Solution for SecondStep {
    type Input = Input;
    type Output = usize;
    fn solve(i: Input) -> Result<usize, ()> {
        Ok(count_bags_in(&i.0["shiny gold"], &i.0))
    }
}

fn count_bags_in(
    bag: &HashMap<String, usize>,
    all_bags: &HashMap<String, HashMap<String, usize>>,
) -> usize {
    bag.iter()
        .map(|(color, count)| {
            count
                * (all_bags
                    .get(color)
                    .map(|subbag| count_bags_in(subbag, all_bags))
                    .unwrap_or(0)
                    + 1)
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::BufReader;

    static INPUT: &str = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."#;

    #[test]
    fn test_parse_bag_line() {
        let mut expected = HashMap::new();
        let mut submap = HashMap::new();
        submap.insert(String::from("bright white"), 1);
        submap.insert(String::from("muted yellow"), 2);
        expected.insert(String::from("light red"), submap);
        let line = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
        let result = parse_bag_line(line);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_step_1() {
        let read = Input::read(BufReader::new(INPUT.as_bytes())).unwrap();
        assert_eq!(Ok(4), FirstStep::solve(read));
    }

    #[test]
    fn test_step_2() {
        let read = Input::read(BufReader::new(INPUT.as_bytes())).unwrap();
        assert_eq!(Ok(32), SecondStep::solve(read));
    }

    #[test]
    fn test_step_2b() {
        let snd_input = r#"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."#;
        let read = Input::read(BufReader::new(snd_input.as_bytes())).unwrap();
        assert_eq!(Ok(126), SecondStep::solve(read));
    }
}
