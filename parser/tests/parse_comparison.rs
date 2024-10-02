use std::vec;

use parser::*;
use tokenizer::tokenize;
fn program_eq(input: &str, expected: Program) {
    let tokens = tokenize(input);

    match parse_program(&tokens) {
        Ok(program) => {
            assert_eq!(program, expected)
        }

        Err(e) => panic!("failed to parse  program:  {:?}", e)
    }
}

#[test]
fn parse_boolean() {
    let input = "x = False";

    let expected = Program {
        statements: vec![
            Stmt::Assignment(
                "x".to_string(),
                Expression::Boolean(false))
        ]
    };

    program_eq(input, expected);

}

#[test]
fn parse_comparison() {
    let input = "x = (5 == 5) && (3 > 2)";

    let expected = Program {
        statements: vec![
            Stmt::Assignment(
                "x".to_string(),
                Expression::CompareOp(
                    Box::new(
                        Expression::CompareOp(
                            Box::new(Expression::Number(5)),
                            CompareOperator::Equal,
                            Box::new(Expression::Number(5))
                        )),
                    CompareOperator::And,
                    Box::new(
                        Expression::CompareOp(
                            Box::new(Expression::Number(3)),
                            CompareOperator::GreaterThan,
                            Box::new(Expression::Number(2))
                        )
                    )
                )
            )
        ]
    };

    program_eq(input, expected);
}