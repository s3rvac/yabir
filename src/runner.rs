//! Running of programs.

use parser::Instruction;
use std;

///
/// A runner of programs.
///
#[derive(Default)]
pub struct Runner {
    /// Program to be run.
    prog: Vec<Instruction>,
    /// A potentially infinite number of data cells.
    data: Vec<u8>,
    /// Instruction pointer (index).
    ip: usize,
    /// Data-cell pointer (index).
    dp: usize
}

impl Runner {
    ///
    /// Constructs a new runner.
    ///
    pub fn new() -> Self {
        Runner::default()
    }

    ///
    /// Runs the given program.
    ///
    /// # Arguments
    ///
    /// * `prog` - Program to be run.
    /// * `input` - Input for the program.
    /// * `output` - Output for the program.
    ///
    pub fn run(&mut self,
               prog: Vec<Instruction>,
               mut input: &mut std::io::Read,
               mut output: &mut std::io::Write) -> Result<(), String> {
        self.prog = prog;
        self.data.clear();
        self.ip = 0;
        self.dp = 0;

        while !self.is_prog_end() {
            try!(self.run_instruction(&mut input, &mut output));
        }

        Ok(())
    }

    fn is_prog_end(&self) -> bool {
        self.ip >= self.prog.len()
    }

    fn run_instruction(&mut self,
                       mut input: &mut std::io::Read,
                       mut output: &mut std::io::Write) -> Result<(), String> {
        match self.prog[self.ip] {
            Instruction::Next => {
                self.dp += 1;
            },
            Instruction::Prev => {
                self.dp -= 1;
            },
            Instruction::Inc => {
                let value = self.get_value();
                self.set_value(value.wrapping_add(1));
            },
            Instruction::Dec => {
                let value = self.get_value();
                self.set_value(value.wrapping_sub(1));
            },
            Instruction::Read => {
                let value = try!(self.read_value(&mut input));
                self.set_value(value);
            },
            Instruction::Write => {
                let value = self.get_value();
                try!(self.write_value(&mut output, value));
            },
            Instruction::LoopStart(loop_end_ip) => {
                if self.is_current_cell_zero() {
                    self.ip = loop_end_ip;
                }
            },
            Instruction::LoopEnd(loop_start_ip) => {
                if !self.is_current_cell_zero() {
                    self.ip = loop_start_ip;
                }
            }
        }
        self.ip += 1;
        Ok(())
    }

    fn get_value(&mut self) -> u8 {
        self.ensure_current_cell_is_accessible();
        self.data[self.dp]
    }

    fn set_value(&mut self, value: u8) {
        self.ensure_current_cell_is_accessible();
        self.data[self.dp] = value;
    }

    fn read_value(&self, mut input: &mut std::io::Read) -> Result<u8, String> {
        let mut buf = [0u8];
        match input.read_exact(&mut buf[..]) {
            Err(err) => Err(format!("{}", err)),
            Ok(_) => Ok(buf[0])
        }
    }

    fn write_value(&self, mut output: &mut std::io::Write, value: u8) -> Result<(), String> {
        match output.write(&[value]) {
            Err(err) => Err(format!("{}", err)),
            Ok(_) => Ok(())
        }
    }

    fn is_current_cell_zero(&mut self) -> bool {
        self.get_value() == 0
    }

    fn ensure_current_cell_is_accessible(&mut self) {
        let cell_count = self.data.len();
        if self.dp >= cell_count {
            for _ in 0..(self.dp - cell_count + 1) {
                self.data.push(0);
            }
        }
    }
}

///
/// Runs the given program.
///
/// # Arguments
///
/// * `prog` - Program to be run.
/// * `input` - Input for the program.
/// * `output` - Output for the program.
///
pub fn run(prog: Vec<Instruction>,
       mut input: &mut std::io::Read,
       mut output: &mut std::io::Write) -> Result<(), String> {
    let mut runner = Runner::new();
    runner.run(prog, &mut input, &mut output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std;
    use parser::Instruction;

    fn assert_run_writes_correct_output(prog: Vec<Instruction>,
                                               input: &[u8],
                                               expected_output: &[u8]) {
        let mut output = Vec::new();
        {
            let mut input_reader = std::io::BufReader::new(input.as_ref());
            let mut output_writer = std::io::BufWriter::new(&mut output);
            run(prog, &mut input_reader, &mut output_writer).unwrap();
        }

        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_run_does_nothing_when_program_has_no_instructions() {
        assert_run_writes_correct_output(
            vec![],
            &[],
            &[]
        );
    }

    #[test]
    fn test_run_can_read_from_input_and_write_to_output() {
        assert_run_writes_correct_output(
            vec![
                Instruction::Read,
                Instruction::Write
            ],
            &[1],
            &[1]
        );
    }

    #[test]
    fn test_run_can_increment_cell() {
        assert_run_writes_correct_output(
            vec![
                Instruction::Read,
                Instruction::Inc,
                Instruction::Write,
            ],
            &[1],
            &[2]
        );
    }

    #[test]
    fn test_run_can_decrement_cell() {
        assert_run_writes_correct_output(
            vec![
                Instruction::Read,
                Instruction::Dec,
                Instruction::Write,
            ],
            &[1],
            &[0]
        );
    }

    #[test]
    fn test_run_can_move_between_cells() {
        assert_run_writes_correct_output(
            vec![
                Instruction::Read,
                Instruction::Next,
                Instruction::Read,
                Instruction::Next,
                Instruction::Prev,
                Instruction::Write,
                Instruction::Prev,
                Instruction::Write,
            ],
            &[1, 2],
            &[2, 1]
        );
    }

    #[test]
    fn test_run_can_loop() {
        assert_run_writes_correct_output(
            vec![
                Instruction::Read,
                Instruction::LoopStart(6),
                Instruction::Dec,
                Instruction::Next,
                Instruction::Inc,
                Instruction::Prev,
                Instruction::LoopEnd(1),
                Instruction::Write,
                Instruction::Next,
                Instruction::Write
            ],
            &[2],
            &[0, 2]
        );
    }
}
