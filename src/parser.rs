//! Syntactic analysis of programs.

use lexer::Token;

///
/// Instructions.
///
#[derive(Debug, PartialEq)]
pub enum Instruction {
    /// Move to the next cell (`>`).
    Next,
    /// Move to the previous cell (`<`
    Prev,
    /// Increment the current cell (`+`).
    Inc,
    /// Decrement the current cell (`-`).
    Dec,
    /// Read a byte from the input into the current cell (`,`).
    Read,
    /// Write a byte from the current cell into the output (`,`).
    Write,
    /// Start of a loop ending on the given index (`[`).
    LoopStart(usize),
    /// End of a loop starting on the given index (`[`).
    LoopEnd(usize),
}

///
/// Parses the given vector of tokens into a vector of instructions.
///
/// # Arguments
///
/// * `tokens` - Tokens representing a program.
///
pub fn parse(tokens: &Vec<Token>) -> Result<Vec<Instruction>, String> {
    let mut instructions = Vec::with_capacity(tokens.len());
    let mut loop_stack: Vec<usize> = Vec::new();
    for (i, token) in tokens.iter().enumerate() {
        match *token {
            Token::Next => instructions.push(Instruction::Next),
            Token::Prev => instructions.push(Instruction::Prev),
            Token::Inc => instructions.push(Instruction::Inc),
            Token::Dec => instructions.push(Instruction::Dec),
            Token::Read => instructions.push(Instruction::Read),
            Token::Write => instructions.push(Instruction::Write),
            Token::LoopStart =>
                try!(handle_loop_start(&mut instructions, &mut loop_stack, i)),
            Token::LoopEnd =>
                try!(handle_loop_end(&mut instructions, &mut loop_stack, i)),
        }
    }
    try!(ensure_all_loops_have_ended(&loop_stack));
    Ok(instructions)
}

fn handle_loop_start(instructions: &mut Vec<Instruction>,
                     loop_stack: &mut Vec<usize>,
                     i: usize)
                     -> Result<(), String> {
    // We don't know the target yet, so pass 0. It will be updated later when
    // we reach the target.
    instructions.push(Instruction::LoopStart(0));
    loop_stack.push(i);
    Ok(())
}

fn handle_loop_end(instructions: &mut Vec<Instruction>,
                   loop_stack: &mut Vec<usize>,
                   i: usize)
                   -> Result<(), String> {
    let target = match loop_stack.pop() {
        Some(target) => target,
        None => {
            return Err(
                format!("missing start of a loop ended at index {}", i)
            );
        }
    };
    instructions.push(Instruction::LoopEnd(target));
    // Update the index of the target in the loop start because it was
    // initialized to zero.
    instructions[target] = Instruction::LoopStart(i);
    Ok(())
}

fn ensure_all_loops_have_ended(loop_stack: &Vec<usize>) -> Result<(), String> {
    if !loop_stack.is_empty() {
        return Err(
            format!(
                "missing end of a loop started at index {}",
                loop_stack.last().unwrap()
            )
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use lexer::Token;

    #[test]
    fn test_parse_returns_empty_vec_when_there_are_no_tokens() {
        assert_eq!(parse(&vec![]).unwrap(), vec![]);
    }

    #[test]
    fn test_parse_correctly_parses_infinite_loop() {
        // []
        let tokens = vec![
            Token::LoopStart,
            Token::LoopEnd,
        ];
        let expected_result = Ok(vec![
            Instruction::LoopStart(1),
            Instruction::LoopEnd(0),
        ]);
        assert_eq!(parse(&tokens), expected_result);
    }

    #[test]
    fn test_parse_correctly_parses_valid_program() {
        // [->+<,]
        let tokens = vec![
            Token::LoopStart,
            Token::Dec,
            Token::Next,
            Token::Inc,
            Token::Prev,
            Token::Read,
            Token::LoopEnd,
        ];
        let expected_result = Ok(vec![
            Instruction::LoopStart(6),
            Instruction::Dec,
            Instruction::Next,
            Instruction::Inc,
            Instruction::Prev,
            Instruction::Read,
            Instruction::LoopEnd(0),
        ]);
        assert_eq!(parse(&tokens), expected_result);
    }

    #[test]
    fn test_parse_returns_error_when_missing_loop_end() {
        assert_eq!(
            parse(&vec![Token::LoopStart]),
            Err("missing end of a loop started at index 0".to_string())
        );
    }

    #[test]
    fn test_parse_returns_error_when_missing_loop_start() {
        assert_eq!(
            parse(&vec![Token::LoopEnd]),
            Err("missing start of a loop ended at index 0".to_string())
        );
    }
}
