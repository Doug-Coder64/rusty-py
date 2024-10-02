use tokenizer::*;

#[derive(Debug, PartialEq)]
pub enum Expression {
    Number(i64),
    String(String),
    Variable(String),
    BinaryOp(Box<Expression>, BinaryOperator, Box<Expression>),
}

#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract, 
    Multiply,
    Divide,
    FloorDivide,
    Modulus,
    Power,
}

#[derive(Debug)]
pub enum PrecedenceLevel {
    AddSub,
    MulDiv,
    Power
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Assignment(String, Expression)
}

#[derive(Debug, PartialEq)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

fn current_token(tokens: &[Token], position: usize) -> &Token {
    tokens.get(position).unwrap_or(&Token::EOF)
}

fn advance(position: &mut usize) {
    *position += 1;
}

fn parse_number(tokens: &[Token], position: &mut usize) -> Option<Expression> {

    if let Token::Number(n) = current_token(tokens, *position) {
        advance(position);
        Some(Expression::Number(*n))
    } else {
        None
    }
}

// Parse Variables (identifiers)
fn parse_identifier(tokens: &[Token], position: &mut usize) -> Option<Expression> {

    if let Token::Identifier(name) = current_token(tokens, *position) {
        advance(position);
        Some(Expression::Variable(name.clone()))
    } else {
        None
    }
}

fn parse_power(tokens: &[Token], position: &mut usize) -> Option<Expression> {

    let mut expr = parse_parenthesize(tokens, position)?;


    while let Token::Operator(op) = current_token(tokens, *position) {
        if op == "**" {
            advance(position);
            let right =  parse_parenthesize(tokens, position)?;
            expr = Expression::BinaryOp(Box::new(expr), BinaryOperator::Power, Box::new(right));
        } else {
            break;
        }
    }

    Some(expr)
   
}

// parse for factors
fn parse_factor(tokens: &[Token], position: &mut usize) -> Option<Expression> {
    let mut expr = parse_power(tokens, position)?;

    while let Token::Operator(op) = current_token(tokens, *position) {
        let operator = match op.as_str() {
            "*" => BinaryOperator::Multiply, 
            "/" => BinaryOperator::Divide,
            "//" => BinaryOperator::FloorDivide,
            "%" => BinaryOperator::Modulus,
            _ => break,
        };
        advance(position);
        let right = parse_power(tokens, position)?;
        expr = Expression::BinaryOp(Box::new(expr), operator, Box::new(right));
    }

    Some(expr)
}


//handles expressions wrapped in parentheses 
fn parse_parenthesize(tokens: &[Token], position: &mut usize) -> Option<Expression> {
    match current_token(tokens, *position) {
        Token::Number(n) => {
            advance(position);
            Some(Expression::Number(*n))
        }, 
        Token::Identifier(name) => {
            advance(position);
            Some(Expression::Variable(name.clone()))
        },
        Token::OpenParen => {
            advance(position);
            let expr = parse_expression(tokens, position)?;

            if let Token::CloseParen = current_token(tokens, *position) {
                advance(position);
                Some(expr)
            } else {
                None
            }
        }, 

        _ => None,
    }
}


fn parse_expression(tokens: &[Token], position: &mut usize) -> Option<Expression> {

    let mut expr = parse_factor(tokens, position)?;
    
    while let Token::Operator(op) = current_token(tokens, *position) {
        let operator = match op.as_str() {
            "+" => BinaryOperator::Add,
            "-" => BinaryOperator::Subtract,
            _ => break,
        };
        advance(position);
        let right =  parse_factor(tokens, position)?;
        expr = Expression::BinaryOp(Box::new(expr), operator, Box::new(right));
    }

    Some(expr)
}

fn parse_assignment(tokens: &[Token], position: &mut usize) -> Option<Stmt> {
    
    if let Some(Expression::Variable(var)) = parse_identifier(tokens, position){
        if let Token::Assign = current_token(tokens, *position) {
            advance(position);
            if let Some(expr) = parse_expression(tokens, position) {
                return Some(Stmt::Assignment(var, expr));
            }
        }
    } 
    
    None  
}

pub fn parse_program(tokens: &[Token]) -> Program {

    let mut position = 0; 
    let mut statements = Vec::new();

    while let Some(stmt) = parse_assignment(tokens, &mut position) {
        statements.push(stmt);
    }

    Program { statements }
}