//! Lexical analysis of programs.

///
/// Valid tokens in programs.
///
#[derive(Debug, PartialEq)]
pub enum Token {
    /// `>`
    Next,
    /// `<`
    Prev,
    /// `+`
    Inc,
    /// `-`
    Dec,
    /// `,`
    Read,
    /// `.`
    Write,
    /// `[`
    LoopStart,
    /// `]`
    LoopEnd,
}

/// A vector of tokens.
pub type Tokens = Vec<Token>;

///
/// Converts the given program into a vector of tokens.
///
/// # Arguments
///
/// * `prog` - A program to be tokenized.
///
pub fn tokenize(prog: &String) -> Tokens {
    let mut tokens = Tokens::new();
    for c in prog.chars() {
        match c {
            '>' => tokens.push(Token::Next),
            '<' => tokens.push(Token::Prev),
            '+' => tokens.push(Token::Inc),
            '-' => tokens.push(Token::Dec),
            ',' => tokens.push(Token::Read),
            '.' => tokens.push(Token::Write),
            '[' => tokens.push(Token::LoopStart),
            ']' => tokens.push(Token::LoopEnd),
            _ => (),
        }
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_returns_empty_vec_when_there_are_no_instructions() {
        assert_eq!(tokenize(&"".to_string()), vec![]);
    }

    #[test]
    fn test_tokenize_recognizes_all_valid_tokens() {
        assert_eq!(
            tokenize(&"><+-,.[]".to_string()),
            vec![
                Token::Next,
                Token::Prev,
                Token::Inc,
                Token::Dec,
                Token::Read,
                Token::Write,
                Token::LoopStart,
                Token::LoopEnd,
            ]
        );
    }

    #[test]
    fn test_tokenize_ignores_unsupported_characters() {
        assert_eq!(tokenize(&"a|()@#".to_string()), vec![]);
    }
}
