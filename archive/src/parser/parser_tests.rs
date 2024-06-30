#[cfg(test)]
mod tests {
    use crate::parser::parser::{new_parser, Parser};
    use crate::token::token::Lexer;

    fn check_parser_errors(parser: &Parser) {
        let errors = &parser.errors;
        if errors.len() == 0 {
            return;
        }
        eprintln!("parser has {} errors", errors.len());
        for error in errors {
            eprintln!("parser error: {}", error);
        }
        panic!("parser has errors");
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;";

        let mut l = Lexer::new(input.to_string());
        let mut p = new_parser(&mut l);

        let program = p.parse_program();
        check_parser_errors(&p);

        if program.Statements.len() != 1 {
            panic!(
                "program has not enough statements. got={}",
                program.Statements.len()
            );
        }
    }
}
