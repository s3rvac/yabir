//
// Project:   yabir
// Copyright: (c) 2016 by Petr Zemek <s3rvac@gmail.com> and contributors
// License:   Apache 2.0 or MIT, see the README file for more details
//

//! Running of programs.

use parser::Instruction;
use parser::Instructions;
use std;

///
/// A runner of programs.
///
#[derive(Default)]
pub struct Runner {
    /// Program to be run.
    prog: Instructions,
    /// A potentially infinite number of data cells.
    data: Vec<u8>,
    /// Instruction pointer (index).
    ip: usize,
    /// Data-cell pointer (index).
    dp: usize,
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
               prog: Instructions,
               mut input: &mut std::io::Read,
               mut output: &mut std::io::Write)
               -> Result<(), String> {
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
                       mut output: &mut std::io::Write)
                       -> Result<(), String> {
        match self.prog[self.ip] {
            Instruction::Next => {
                try!(self.ensure_can_increment_dp_by_one());
                self.dp += 1;
            }
            Instruction::Prev => {
                try!(self.ensure_can_decrement_dp_by_one());
                self.dp -= 1;
            }
            Instruction::Inc => {
                let value = self.load_value();
                self.store_value(value.wrapping_add(1));
            }
            Instruction::Dec => {
                let value = self.load_value();
                self.store_value(value.wrapping_sub(1));
            }
            Instruction::Read => {
                try!(self.read_value(&mut input));
            }
            Instruction::Write => {
                try!(self.write_value(&mut output));
            }
            Instruction::LoopStart(loop_end_ip) => {
                if self.is_current_cell_zero() {
                    self.ip = loop_end_ip;
                }
            }
            Instruction::LoopEnd(loop_start_ip) => {
                if !self.is_current_cell_zero() {
                    self.ip = loop_start_ip;
                }
            }
        }
        self.ip += 1;
        Ok(())
    }

    fn load_value(&mut self) -> u8 {
        self.ensure_current_cell_is_accessible();
        self.data[self.dp]
    }

    fn store_value(&mut self, value: u8) {
        self.ensure_current_cell_is_accessible();
        self.data[self.dp] = value;
    }

    fn read_value(&mut self, mut input: &mut std::io::Read) -> Result<(), String> {
        let mut buf = [0u8];
        match input.read_exact(&mut buf) {
            Err(err) => match err.kind() {
                std::io::ErrorKind::UnexpectedEof => {
                    // There are multiple ways of dealing with EOF (see
                    // https://en.wikipedia.org/wiki/Brainfuck#End-of-file_behavior).
                    // We have chosen to leave the current cell unchanged.
                    Ok(())
                },
                _ => Err(format!("reading of a value failed (reason: {:?})", err.kind())),
            },
            Ok(_) => {
                self.store_value(buf[0]);
                Ok(())
            }
        }
    }

    fn write_value(&mut self, mut output: &mut std::io::Write) -> Result<(), String> {
        let value = self.load_value();
        match output.write(&[value]) {
            Err(err) => Err(
                format!("writing of a value failed (reason: {})", err)
            ),
            Ok(_) => Ok(()),
        }
    }

    fn is_current_cell_zero(&mut self) -> bool {
        self.load_value() == 0
    }

    fn ensure_current_cell_is_accessible(&mut self) {
        let cell_count = self.data.len();
        if self.dp >= cell_count {
            for _ in 0..(self.dp - cell_count + 1) {
                self.data.push(0);
            }
        }
    }

    fn ensure_can_increment_dp_by_one(&self) -> Result<(), String> {
        match self.dp {
            std::usize::MAX => Err(
                "cannot increment the data pointer because it is MAX".to_string()
            ),
            _ => Ok(()),
        }
    }

    fn ensure_can_decrement_dp_by_one(&self) -> Result<(), String> {
        match self.dp {
            0 => Err("cannot decrement the data pointer because it is 0".to_string()),
            _ => Ok(()),
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
pub fn run(prog: Instructions,
           mut input: &mut std::io::Read,
           mut output: &mut std::io::Write)
           -> Result<(), String> {
    let mut runner = Runner::new();
    runner.run(prog, &mut input, &mut output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std;
    use parser::Instruction;
    use parser::Instructions;

    fn run_and_get_output(prog: Instructions,
                          input: &[u8])
                          -> Result<Vec<u8>, String> {
        let mut output = Vec::new();
        match {
            let mut input_reader = std::io::BufReader::new(input.as_ref());
            let mut output_writer = std::io::BufWriter::new(&mut output);
            run(prog, &mut input_reader, &mut output_writer)
        } {
            Ok(_) => Ok(output),
            Err(err) => Err(err),
        }
    }

    fn assert_run_writes_correct_output(prog: Instructions,
                                        input: &[u8],
                                        expected_output: &[u8]) {
        let output = match run_and_get_output(prog, input) {
            Ok(output) => output,
            Err(err) => panic!("run() failed with {}", err),
        };
        assert_eq!(output, expected_output);
    }

    fn assert_run_returns_error(prog: Instructions,
                         input: &[u8]) {
        let result = run_and_get_output(prog, input);
        assert!(result.is_err());
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
    fn test_read_does_not_alter_cell_value_when_at_end_of_file() {
        assert_run_writes_correct_output(
            vec![
                Instruction::Inc,
                Instruction::Read,
                Instruction::Write
            ],
            &[],
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

    #[test]
    fn test_copies_input_to_output() {
        assert_run_writes_correct_output(
            vec![
                // Read the whole input:
                Instruction::Next,
                Instruction::Inc,
                Instruction::LoopStart(5),
                Instruction::Next,
                Instruction::Read,
                Instruction::LoopEnd(2),
                // Rewind to the beginning:
                Instruction::Prev,
                Instruction::LoopStart(9),
                Instruction::Prev,
                Instruction::LoopEnd(7),
                // Write the whole input:
                Instruction::Next,
                Instruction::Next,
                Instruction::LoopStart(15),
                Instruction::Write,
                Instruction::Next,
                Instruction::LoopEnd(12)
            ],
            &[4, 3, 1, 5],
            &[4, 3, 1, 5]
        );
    }

    #[test]
    fn test_returns_error_when_underflowing_data_pointer() {
        assert_run_returns_error(vec![Instruction::Prev], &[]);
    }
}
