use anyhow::{anyhow, Result};

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Add(Parameter, Parameter, Parameter),
    Multiply(Parameter, Parameter, Parameter),
    Input(Parameter),
    Output(Parameter),
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, Parameter),
    Equals(Parameter, Parameter, Parameter),
    AdjustRelativeBase(Parameter),
    Halt,
}

#[derive(Debug, Copy, Clone)]
enum Parameter {
    Immediate(i64),
    Indexed(IndexedParameter),
}

#[derive(Debug, Copy, Clone)]
pub enum IndexedParameter {
    Positional(usize),
    Relative(usize),
}

#[derive(Debug, Clone)]
pub struct Computer {
    memory: Vec<i64>,
    pointer: usize,
    relative_base: usize,
}

#[derive(Debug, Copy, Clone)]
enum ExecutionResult {
    Running(Option<i64>),
    Stopped(StoppedResult),
}

#[derive(Debug, Copy, Clone)]
pub enum StoppedResult {
    Blocked,
    Halted,
}

impl Computer {
    pub fn new(initial_memory: &Vec<i64>) -> Computer {
        let mut internal_memory = initial_memory.clone();
        internal_memory.resize(2048, 0);

        Computer {
            memory: internal_memory,
            pointer: 0,
            relative_base: 0,
        }
    }

    pub fn new_from_str(serialized_memory: &str) -> Result<Computer> {
        let initial_memory = serialized_memory
            .trim()
            .split(',')
            .map(|x| x.parse::<i64>())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self::new(&initial_memory))
    }

    pub fn run(&mut self, input: Vec<i64>) -> Result<Vec<i64>> {
        let (result, output) = self.run_until_blocked(input)?;
        match result {
            StoppedResult::Blocked => Err(anyhow!("Blocked on input")),
            StoppedResult::Halted => Ok(output),
        }
    }

    pub fn run_until_blocked(&mut self, input: Vec<i64>) -> Result<(StoppedResult, Vec<i64>)> {
        let mut input_stream = input.into_iter();
        let mut output = vec![];

        loop {
            let instruction = self.get_instruction()?;
            let result = self.do_instruction(instruction, &mut input_stream);

            match result {
                ExecutionResult::Running(None) => {}
                ExecutionResult::Running(Some(val)) => {
                    output.push(val);
                }
                ExecutionResult::Stopped(stopped_result) => {
                    return Ok((stopped_result, output));
                }
            }
        }
    }

    pub fn get_memory_value(&self, idx: usize) -> i64 {
        self.memory[idx]
    }

    pub fn set_value(&mut self, param: IndexedParameter, value: i64) {
        match param {
            IndexedParameter::Positional(idx) => {
                self.set_memory_value(idx, value);
            }
            IndexedParameter::Relative(offset) => {
                let idx = self.relative_base.wrapping_add(offset);
                self.set_memory_value(idx, value);
            }
        }
    }

    fn do_instruction(
        &mut self,
        instruction: Instruction,
        mut input_stream: impl Iterator<Item = i64>,
    ) -> ExecutionResult {
        match instruction {
            Instruction::Add(a, b, Parameter::Indexed(p)) => {
                self.set_value(p, self.get_param_value(a) + self.get_param_value(b));
                self.pointer += 4;
                ExecutionResult::Running(None)
            }
            Instruction::Multiply(a, b, Parameter::Indexed(p)) => {
                self.set_value(p, self.get_param_value(a) * self.get_param_value(b));
                self.pointer += 4;
                ExecutionResult::Running(None)
            }
            Instruction::Input(Parameter::Indexed(p)) => match input_stream.next() {
                Some(input) => {
                    self.set_value(p, input);
                    self.pointer += 2;
                    ExecutionResult::Running(None)
                }
                None => ExecutionResult::Stopped(StoppedResult::Blocked),
            },
            Instruction::Output(a) => {
                let output = self.get_param_value(a);
                self.pointer += 2;
                ExecutionResult::Running(Some(output))
            }
            Instruction::JumpIfTrue(a, b) => {
                if self.get_param_value(a) != 0 {
                    self.pointer = self.get_param_value(b) as usize;
                } else {
                    self.pointer += 3;
                }
                ExecutionResult::Running(None)
            }
            Instruction::JumpIfFalse(a, b) => {
                if self.get_param_value(a) == 0 {
                    self.pointer = self.get_param_value(b) as usize;
                } else {
                    self.pointer += 3;
                }
                ExecutionResult::Running(None)
            }
            Instruction::LessThan(a, b, Parameter::Indexed(p)) => {
                let value = match self.get_param_value(a) < self.get_param_value(b) {
                    true => 1,
                    false => 0,
                };

                self.set_value(p, value);
                self.pointer += 4;
                ExecutionResult::Running(None)
            }
            Instruction::Equals(a, b, Parameter::Indexed(p)) => {
                let value = match self.get_param_value(a) == self.get_param_value(b) {
                    true => 1,
                    false => 0,
                };

                self.set_value(p, value);
                self.pointer += 4;
                ExecutionResult::Running(None)
            }
            Instruction::AdjustRelativeBase(a) => {
                self.relative_base = self
                    .relative_base
                    .wrapping_add(self.get_param_value(a) as usize);

                self.pointer += 2;
                ExecutionResult::Running(None)
            }
            Instruction::Halt => ExecutionResult::Stopped(StoppedResult::Halted),
            _ => unreachable!(),
        }
    }

    fn get_instruction(&self) -> Result<Instruction> {
        let compact_opcode = self.get_memory_value(self.pointer) % 100;
        match compact_opcode {
            1 => Ok(Instruction::Add(
                self.get_param(0)?,
                self.get_param(1)?,
                self.get_param(2)?,
            )),
            2 => Ok(Instruction::Multiply(
                self.get_param(0)?,
                self.get_param(1)?,
                self.get_param(2)?,
            )),
            3 => Ok(Instruction::Input(self.get_param(0)?)),
            4 => Ok(Instruction::Output(self.get_param(0)?)),
            5 => Ok(Instruction::JumpIfTrue(
                self.get_param(0)?,
                self.get_param(1)?,
            )),
            6 => Ok(Instruction::JumpIfFalse(
                self.get_param(0)?,
                self.get_param(1)?,
            )),
            7 => Ok(Instruction::LessThan(
                self.get_param(0)?,
                self.get_param(1)?,
                self.get_param(2)?,
            )),
            8 => Ok(Instruction::Equals(
                self.get_param(0)?,
                self.get_param(1)?,
                self.get_param(2)?,
            )),
            9 => Ok(Instruction::AdjustRelativeBase(self.get_param(0)?)),
            99 => Ok(Instruction::Halt),
            _ => Err(anyhow!("Unrecognized opcode {}", compact_opcode)),
        }
    }

    fn get_param(&self, offset: usize) -> Result<Parameter> {
        let compact_instruction = self.get_memory_value(self.pointer);
        let flag = (compact_instruction / (10i64.pow(offset as u32 + 2))) % 10;

        let data = self.get_memory_value(self.pointer + offset + 1);
        match flag {
            0 => Ok(Parameter::Indexed(IndexedParameter::Positional(
                data as usize,
            ))),
            1 => Ok(Parameter::Immediate(data)),
            2 => Ok(Parameter::Indexed(IndexedParameter::Relative(
                data as usize,
            ))),
            _ => Err(anyhow!("Unrecognized parameter flag {}", flag)),
        }
    }

    fn get_param_value(&self, param: Parameter) -> i64 {
        match param {
            Parameter::Indexed(IndexedParameter::Positional(idx)) => self.get_memory_value(idx),
            Parameter::Immediate(data) => data,
            Parameter::Indexed(IndexedParameter::Relative(offset)) => {
                self.get_memory_value(self.relative_base.wrapping_add(offset))
            }
        }
    }

    fn set_memory_value(&mut self, idx: usize, value: i64) {
        self.memory[idx] = value;
    }
}
