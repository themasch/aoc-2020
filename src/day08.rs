use crate::*;
use std::io::BufRead;

#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

#[derive(Debug)]
pub struct Input(Vec<Instruction>);

impl<R: BufRead> ReadInput<R> for Input {
    fn read(b: R) -> Result<Input, ()> {
        let vec = b
            .lines()
            .flatten()
            .flat_map(|line| parse_line(&line).ok())
            .collect();
        Ok(Input(vec))
    }
}

fn parse_line(line: &str) -> Result<Instruction, ()> {
    match &line[0..3] {
        "acc" => Ok(Instruction::Acc(line[4..].parse().unwrap())),
        "jmp" => Ok(Instruction::Jmp(line[4..].parse().unwrap())),
        "nop" => Ok(Instruction::Nop(line[4..].parse().unwrap())),
        _ => {
            dbg!(&line[0..3]);
            Err(())
        }
    }
}

use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Machine {
    acc: isize,
    pc: usize,
    visited: HashSet<usize>,
    prog: Vec<Instruction>,
}

#[derive(Debug)]
enum ProgResult {
    Crash(isize),
    Terminated(isize),
}

impl Machine {
    fn new(prog: Vec<Instruction>) -> Machine {
        Machine {
            acc: 0,
            pc: 0,
            visited: HashSet::new(),
            prog,
        }
    }

    fn step(&mut self) -> Option<ProgResult> {
        if self.visited.contains(&self.pc) {
            return Some(ProgResult::Crash(self.acc));
        }

        if self.pc >= self.prog.len() {
            return Some(ProgResult::Terminated(self.acc));
        }

        self.visited.insert(self.pc);

        match self.prog.get(self.pc) {
            Some(Instruction::Acc(sum)) => {
                self.acc += sum;
                self.pc += 1;
            }
            Some(Instruction::Jmp(steps)) => self.pc = (self.pc as isize + steps) as usize,
            Some(Instruction::Nop(_)) => self.pc += 1,
            None => panic!("invalid program counter: {}", self.pc),
        };

        None
    }

    fn peek_instruction(&self) -> Option<&Instruction> {
        self.prog.get(self.pc)
    }

    fn run(&mut self) -> ProgResult {
        loop {
            if let Some(res) = self.step() {
                return res;
            }
        }
    }
}

struct ProgramFixer;

impl ProgramFixer {
    fn fix(mut machine: Machine) -> Vec<Instruction> {
        loop {
            match machine.peek_instruction() {
                Some(Instruction::Nop(range)) => {
                    let mut alternate_version = machine.clone();
                    alternate_version.prog[machine.pc] = Instruction::Jmp(*range);
                    if let ProgResult::Terminated(_) = alternate_version.run() {
                        return alternate_version.prog;
                    }
                    machine.step();
                }
                Some(Instruction::Jmp(range)) => {
                    let mut alternate_version = machine.clone();
                    alternate_version.prog[machine.pc] = Instruction::Nop(*range);
                    if let ProgResult::Terminated(_) = alternate_version.run() {
                        return alternate_version.prog;
                    }
                    machine.step();
                }
                Some(Instruction::Acc(_)) => {
                    if let Some(res) = machine.step() {
                        dbg!(res);
                        panic!();
                    }
                }
                None => {
                    println!("programm finished at {}", machine.pc);
                    panic!();
                }
            }
        }
    }
}

pub struct FirstStep;
impl Solution for FirstStep {
    type Input = Input;
    type Output = isize;
    fn solve(i: Input) -> Result<isize, ()> {
        let mut machine = Machine::new(i.0);
        if let ProgResult::Crash(result) = machine.run() {
            Ok(result)
        } else {
            panic!("{:?}",)
        }
    }
}

pub struct SecondStep;
impl Solution for SecondStep {
    type Input = Input;
    type Output = isize;
    fn solve(i: Input) -> Result<isize, ()> {
        let machine = Machine::new(i.0);
        let fixed_code = ProgramFixer::fix(machine);
        let mut fixed_machine = Machine::new(fixed_code);
        if let ProgResult::Terminated(result) = fixed_machine.run() {
            Ok(result)
        } else {
            panic!("{:?}",)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::BufReader;

    static TEST_INPUT: &str = r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#;

    #[test]
    fn test_asm_parser() {
        use Instruction::*;
        assert_eq!(Ok(Nop(0)), parse_line("nop +0"));
        assert_eq!(Ok(Nop(0)), parse_line("nop -0"));
        assert_eq!(Ok(Nop(-99)), parse_line("nop -99"));
        assert_eq!(Ok(Acc(-99)), parse_line("acc -99"));
        assert_eq!(Ok(Acc(0)), parse_line("acc -0"));
        assert_eq!(Ok(Acc(0)), parse_line("acc -0"));
        assert_eq!(Ok(Acc(10)), parse_line("acc +10"));
        assert_eq!(Ok(Jmp(10)), parse_line("jmp +10"));
        assert_eq!(Ok(Jmp(-10)), parse_line("jmp -10"));
    }

    #[test]
    fn test_step_1() {
        let read = Input::read(BufReader::new(TEST_INPUT.as_bytes())).unwrap();
        assert_eq!(Ok(5), FirstStep::solve(read));
    }

    #[test]
    fn test_step_2() {
        let read = Input::read(BufReader::new(TEST_INPUT.as_bytes())).unwrap();
        assert_eq!(Ok(8), SecondStep::solve(read));
    }
}
