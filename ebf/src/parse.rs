#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Statement {
    Add(u32),
    Sub(u32),
    LoopStart,
    LoopEnd,
    ShiftLeft(u32),
    ShiftRight(u32),
    Input,
    Output,
}

pub fn parse_ebf(code: String) -> Vec<Statement> {
    let tokens = code.split_ascii_whitespace();
    let mut statements = Vec::new();

    for token in tokens {
        let mut cs = token.chars();
        match cs.next().unwrap() {
            '+' => statements.push(Statement::Add(cs.collect::<String>().parse::<u32>().unwrap_or(1))),
            '-' => statements.push(Statement::Sub(cs.collect::<String>().parse::<u32>().unwrap_or(1))),
            '[' => statements.push(Statement::LoopStart),
            ']' => statements.push(Statement::LoopEnd),
            '<' => statements.push(Statement::ShiftLeft(cs.collect::<String>().parse::<u32>().unwrap_or(1))),
            '>' => statements.push(Statement::ShiftRight(cs.collect::<String>().parse::<u32>().unwrap_or(1))),
            ',' => statements.push(Statement::Input),
            '.' => statements.push(Statement::Output),
            '#' => (),
            _ => panic!("Unknown token: {}", token),
        }
    }

    return statements;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parsing() {
        let code = "+5 <2 [ #start_of_loop + . ]".to_string();
        let statements = super::parse_ebf(code);
        dbg!(&statements);
        assert_eq!(statements.len(), 6);
    }
}
