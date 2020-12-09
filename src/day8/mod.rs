use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;

use crate::AdventError;

pub fn part1() -> Result<(), AdventError> {
    let input = include_str!("input");
    let mut machine = input.parse::<AccumulatorMachine>().unwrap();
    let output = machine.run_until_duplicate().unwrap();
    println!("The accumulator is {:?}", output);
    Ok(())
}

pub fn part2() -> Result<(), AdventError> {
    let input = include_str!("input");
    let machine = input.parse::<AccumulatorMachine>().unwrap();
    for i in 0..machine.program.len() {
        let instruction = machine.program[i];
        match instruction {
            OpCode::Nop(p) => {
                let mut test_machine = machine.clone();
                test_machine.program[i] = OpCode::Jmp(p);
                let possible_result = test_machine.run_until_duplicate();
                if let Ok((acc, terminated)) = possible_result {
                    if terminated {
                        println!("Program terminated with acc = {} after changing {} to Jmp", acc, i);
                    }
                }
            },
            OpCode::Jmp(p) => {
                let mut test_machine = machine.clone();
                test_machine.program[i] = OpCode::Nop(p);
                let possible_result = test_machine.run_until_duplicate();
                if let Ok((acc, terminated)) = possible_result {
                    if terminated {
                        println!("Program terminated with acc = {} after changing {} to Nop", acc, i);
                    }
                }
            },
            OpCode::Acc(_) => continue
        }
    }
    Ok(())
}
#[derive(Debug, Clone, Copy)]
enum OpCode {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

#[derive(Debug)]
struct InstructionParseError {
    cause: String,
    input: String,
}

impl From<ParseIntError> for InstructionParseError {
    fn from(p: ParseIntError) -> Self {
        InstructionParseError{
            cause: format!("unable to parse parameter {}", p),
            input: "???".to_string(),
        }
    }
}

impl FromStr for AccumulatorMachine {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let program: Result<Vec<OpCode>, InstructionParseError> = 
                s.lines()
                 .map(|l| l.parse::<OpCode>())
                 .collect();

        Ok(AccumulatorMachine{
            program: program?,
            instruction_pointer: 0,
            accumulator: 0,
        })
    }
}

impl FromStr for OpCode {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_ascii_whitespace();
        let opcode = parts.next().ok_or(InstructionParseError{cause: "missing opcode".to_string(), input: s.to_string()})?;
        let parameter = parts.next().ok_or(InstructionParseError{cause: "missing parameter".to_string(), input: s.to_string()})?;
        let parameter = parameter.parse::<i32>()?;
        match opcode {
            "acc" => Ok(OpCode::Acc(parameter)),
            "jmp" => Ok(OpCode::Jmp(parameter)),
            "nop" => Ok(OpCode::Nop(parameter)),
            _ => Err(InstructionParseError{cause: "unknown opcode".to_string(), input: s.to_string()})
        }
    }
}

#[derive(Debug, Clone)]
struct AccumulatorMachine {
    program: Vec<OpCode>,
    instruction_pointer: i32,
    accumulator: i32,
}

#[derive(Debug)]
enum ExecutionErrorKind {
    InstructionOutOfBounds,
}

#[derive(Debug)]
struct AccumulatorExecutionError {
    kind: ExecutionErrorKind,
}

impl AccumulatorMachine {
    fn run_until_duplicate(&mut self) -> Result<(i32, bool), AccumulatorExecutionError> {
        let mut executed: HashSet<i32> = HashSet::new();
        loop {
            if executed.contains(&self.instruction_pointer) {
                return Ok((self.accumulator, false));
            }
            executed.insert(self.instruction_pointer);
            let instruction = self.program[self.instruction_pointer as usize];
            // println!("{}: {:?} / {}", self.instruction_pointer, instruction, self.accumulator);
            if let Some(err) = self.exec(instruction) {
                //println!("duplication!");
                return Err(err);
            }
            if self.instruction_pointer == self.program.len() as i32 {
                println!("termination!");
                return Ok((self.accumulator, true));
            }
        }
    }
    fn exec(&mut self, op_code: OpCode) -> Option<AccumulatorExecutionError> {
        match op_code {
            OpCode::Acc(p) => {
                self.accumulator += p;
                self.instruction_pointer += 1;
                None
            }
            OpCode::Jmp(p) => {
                let next_ip = self.instruction_pointer + p;
                if next_ip <= 0 || (next_ip as usize) > self.program.len() {
                    return Some(AccumulatorExecutionError{ 
                        kind: ExecutionErrorKind::InstructionOutOfBounds,
                    })
                }
                self.instruction_pointer = next_ip;
                None
            }
            OpCode::Nop(_) => {
                self.instruction_pointer += 1;
                None
            }
        }
    }
}