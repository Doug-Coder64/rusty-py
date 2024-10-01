use parser::*;

fn program_eq(input: &str, expected: Program) {
    match parse_program(input) {
        Ok((_, program)) => {
            assert_eq!(program, expected);
        }

        Err(e) => panic!("Failed to parse program: {:?}", e),
    }
}


#[test]
fn addition() {
    let input  = "x = 1 + 2";
    let expected = Program {
        statements: vec![
            Stmt::Assignment(
                "x".to_string(), 
                Expression::BinaryOp(
                    Box::new(Expression::Number(1)),
                    BinaryOperator::Add,
                    Box::new(Expression::Number(2))
                )
            )
        ]
    };

    program_eq(input, expected);
}

#[test]
fn subtract() {
    let input  = "x = 1 - 2";
    let expected = Program {
        statements: vec![
            Stmt::Assignment(
                "x".to_string(), 
                Expression::BinaryOp(
                    Box::new(Expression::Number(1)),
                    BinaryOperator::Subtract,
                    Box::new(Expression::Number(2))
                )
            )
        ]
    };

    program_eq(input, expected);
}

#[test]
fn multiply() {
    let input  = "x = 1 * 2";
    let expected = Program {
        statements: vec![
            Stmt::Assignment(
                "x".to_string(), 
                Expression::BinaryOp(
                    Box::new(Expression::Number(1)),
                    BinaryOperator::Multiply,
                    Box::new(Expression::Number(2))
                )
            )
        ]
    };

    program_eq(input, expected);
}

#[test]
fn divide() {
    let input  = "x = 1 / 2";
    let expected = Program {
        statements: vec![
            Stmt::Assignment(
                "x".to_string(), 
                Expression::BinaryOp(
                    Box::new(Expression::Number(1)),
                    BinaryOperator::Divide,
                    Box::new(Expression::Number(2))
                )
            )
        ]
    };

    program_eq(input, expected);
}

#[test]
fn floor_divide() {
    let input = "x = 1 // 2";

    let expected = Program {
        statements: vec![
            Stmt::Assignment(
                "x".to_string(), 
                Expression::BinaryOp(
                        Box::new(Expression::Number(1)),
                        BinaryOperator::FloorDivide, 
                        Box::new(Expression::Number(2))
                    )
                )
        ]
    };

    program_eq(input, expected);
    
}

#[test]
fn modulus() {
    let input = "x = 1 % 2";

    let expected = Program {
        statements: vec![
            Stmt::Assignment(
                "x".to_string(), 
                Expression::BinaryOp(
                        Box::new(Expression::Number(1)),
                        BinaryOperator::Modulus, 
                        Box::new(Expression::Number(2))
                    )
                )
        ]
    };

    program_eq(input, expected);
    
}

#[test]
fn power() {
    let input = "x = 1 ** 2";
    
    let expected = Program {
        statements: vec![
            Stmt::Assignment(
                "x".to_string(), 
                Expression::BinaryOp(
                    Box::new(Expression::Number(1)),
                    BinaryOperator::Power, 
                    Box::new(Expression::Number(2))
                )
            )
        ]
    };

    program_eq(input, expected);
}


#[test]
fn parentheses() {
    let input = "x= (1 + 2) * 3";

    let expected = Program {
        statements: vec![
            Stmt::Assignment(
                "x".to_string(),
                Expression::BinaryOp(
                    Box::new(Expression::BinaryOp(
                        Box::new(Expression::Number(1)),
                        BinaryOperator::Add,
                        Box::new(Expression::Number(2))
                    )),
                    BinaryOperator::Multiply,
                    Box::new(Expression::Number(3))
                ),
            ),
        ]
    };

    program_eq(input, expected);
}

#[test]
fn order_operations() {
    let input = "x = 1 + 2 - 3 * 4 / 5 // 6 % 7 ** (8 + 9)";

    let expected = Program {
        statements: vec![
            Stmt::Assignment(
                "x".to_string(), 
                Expression::BinaryOp(
                    Box::new(Expression::BinaryOp(
                            Box::new(Expression::Number(1)),
                            BinaryOperator::Add,
                            Box::new(Expression::Number(2))
                        )
                    ), 
                    BinaryOperator::Subtract,
                    Box::new(Expression::BinaryOp(
                        Box::new(Expression::BinaryOp(
                            Box::new(Expression::BinaryOp(
                                Box::new(Expression::BinaryOp(
                                    Box::new(Expression::Number(3)),
                                    BinaryOperator::Multiply,
                                    Box::new(Expression::Number(4))
                                )
                            ), 
                            BinaryOperator::Divide,
                            Box::new(Expression::Number(5))
                        )),
                        BinaryOperator::FloorDivide,
                        Box::new(Expression::Number(6))
                    )),
                    BinaryOperator::Modulus,
                    Box::new(Expression::BinaryOp(
                        Box::new(Expression::Number(7)),
                        BinaryOperator::Power,
                        Box::new(Expression::BinaryOp(
                            Box::new(Expression::Number(8)),
                            BinaryOperator::Add,
                            Box::new(Expression::Number(9))
                        ))
                    ))
                ))    
            ))
        ]
    };

    program_eq(input, expected);
}

#[test]
fn multi_line() {
    let input = "x= 1 + 2 * -3\ny = 4 - 5\nz = 3 % 4";

    let expected = Program {
        statements: vec![
            Stmt::Assignment(
                "x".to_string(),
                Expression::BinaryOp(
                    Box::new(Expression::Number(1)),
                    BinaryOperator::Add, 
                    Box::new(Expression::BinaryOp(
                        Box::new(Expression::Number(2)),
                        BinaryOperator::Multiply,
                        Box::new(Expression::Number(-3))
                    )),
                ),
            ),
            Stmt::Assignment(
                "y".to_string(),
                Expression::BinaryOp(
                    Box::new(Expression::Number(4)),
                    BinaryOperator::Subtract,
                    Box::new(Expression::Number(5))
                ),
            ), 
            Stmt::Assignment(
                "z".to_string(),
                Expression::BinaryOp(
                    Box::new(Expression::Number(3)),
                    BinaryOperator::Modulus,
                    Box::new(Expression::Number(4))
                ),
            ),  
        ],
    };

    program_eq(input, expected);
}
