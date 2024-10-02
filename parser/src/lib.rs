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

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(String),
    MismatchParenthesis,
    UnexpectedEOF,
    InvalidAssignment,
    InvalidIdentifier,
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

// Parse Variables (identifiers)
fn parse_identifier(tokens: &[Token], position: &mut usize) -> Result<Expression, ParseError> {

    if let Token::Identifier(name) = current_token(tokens, *position) {
        advance(position);
        Ok(Expression::Variable(name.clone()))
    } else {
        Err(ParseError::InvalidIdentifier)
    }
}

fn parse_power(tokens: &[Token], position: &mut usize) -> Result<Expression, ParseError> {

    let mut expr = parse_primary(tokens, position)?;


    while let Token::Operator(op) = current_token(tokens, *position) {
        if op == "**" {
            advance(position);
            let right =  parse_primary(tokens, position)?;
            expr = Expression::BinaryOp(Box::new(expr), BinaryOperator::Power, Box::new(right));
        } else {
            break;
        }
    }

    Ok(expr)
   
}

// parse for factors
fn parse_factor(tokens: &[Token], position: &mut usize) -> Result<Expression, ParseError> {
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

    Ok(expr)
}


//handles expressions wrapped in parentheses 
fn parse_primary(tokens: &[Token], position: &mut usize) -> Result<Expression, ParseError> {
    match current_token(tokens, *position) {
        Token::Number(n) => {
            advance(position);
            Ok(Expression::Number(*n))
        }, 
        Token::Identifier(name) => {
            advance(position);
            Ok(Expression::Variable(name.clone()))
        },
        Token::String(value) => {
            advance(position);
            Ok(Expression::String(value.clone()))
        }
        Token::OpenParen => {
            advance(position);
            let expr = parse_expression(tokens, position)?;

            if let Token::CloseParen = current_token(tokens, *position) {
                advance(position);
                Ok(expr)
            } else {
                Err(ParseError::MismatchParenthesis)
            }
        }, 

        _ => Err(ParseError::UnexpectedToken(format!("{:?}", current_token(tokens, *position)))),
    }
}


fn parse_expression(tokens: &[Token], position: &mut usize) -> Result<Expression, ParseError> {

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

    Ok(expr)
}

fn parse_assignment(tokens: &[Token], position: &mut usize) -> Result<Stmt, ParseError> {
    
    if let Expression::Variable(var) = parse_identifier(tokens, position)? {
        if let Token::Assign = current_token(tokens, *position) {
            advance(position);
            let expr = parse_expression(tokens, position)?;
            return Ok(Stmt::Assignment(var, expr));
            
        } else {
            return Err(ParseError::InvalidAssignment);
        }
    } 
    
    Err(ParseError::InvalidIdentifier)  
}

pub fn parse_program(tokens: &[Token]) -> Result<Program, ParseError> {

    let mut position = 0; 
    let mut statements = Vec::new();

    while position < tokens.len() {
        if let Token::EOF = current_token(tokens, position) {
            break;
        }

        match parse_assignment(tokens, &mut position) {
            Ok(stmt) => statements.push(stmt),
            Err(e) => return Err(e),
        }
    }

    Ok(Program { statements })
    
}