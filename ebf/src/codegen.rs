use crate::parse::Statement;

static OPTIMIZATIONS: [&'static str; 4] = [
    "+-", "-+", "<>", "><",
];

pub fn gen_bf(ast: Vec<Statement>) -> String {
    let mut code = String::new();
    for stmt in ast {
        match stmt {
            Statement::Add(c) => {
                code.push_str(&*"+".repeat(c as usize));
            }
            Statement::Sub(c) => {
                code.push_str(&*"-".repeat(c as usize));
            }
            Statement::LoopStart => {
                code.push('[');
            }
            Statement::LoopEnd => {
                code.push(']');
            }
            Statement::ShiftLeft(c) => {
                code.push_str(&*"<".repeat(c as usize));
            }
            Statement::ShiftRight(c) => {
                code.push_str(&*">".repeat(c as usize));
            }
            Statement::Input => {
                code.push(',');
            }
            Statement::Output => {
                code.push('.');
            }
        }
    }

    for o in OPTIMIZATIONS {
        while code.contains(o) {
            code.remove_matches(o);
        }
    }

    code
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parsing() {
        let code = "+5 -9 <2 > [ + . ] -3".to_string();
        let statements = crate::parse::parse_ebf(code);
        let gen = super::gen_bf(statements);
        assert_eq!(gen, "----<[+.]---");
    }
}
