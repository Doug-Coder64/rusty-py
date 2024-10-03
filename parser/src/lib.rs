use tokenizer::*;

#[derive(Debug, PartialEq)]
pub enum Expression {
    Number(i64),
    String(String),
    Variable(String),
    Boolean(bool),
    BinaryOp(Box<Expression>, BinaryOperator, Box<Expression>),
    CompareOp(Box<Expression>, CompareOperator, Box<Expression>),
    FunctionCall(String, Vec<Expression>),
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

#[derive(Debug, PartialEq)]
pub enum CompareOperator {
    Equal,
    NotEqual,
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,
    And,
    Or,
}


#[derive(Debug)]
pub enum PrecedenceLevel {
    AddSub,
    MulDiv,
    Power
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Assignment(String, Expression),
    FunctionDef(String, Vec<String>, Vec<Stmt>),
    Return(Expression),
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
        Token::Boolean(value) => {
            advance(position);
            Ok(Expression::Boolean(*value))
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
    parse_boolean(tokens, position)
}

fn parse_add_sub(tokens: &[Token], position: &mut usize) -> Result<Expression, ParseError> {
    let mut expr = parse_comparison(tokens, position)?;

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

pub fn parse_comparison(tokens: &[Token], position: &mut usize) -> Result<Expression, ParseError> {
    let mut expr = parse_factor(tokens, position)?;

    while let Token::Operator(op) = current_token(tokens, *position) {
        let operator = match op.as_str() {
            "==" => CompareOperator::Equal,
            "!=" => CompareOperator::NotEqual,
            "<" => CompareOperator::LessThan,
            "<=" => CompareOperator::LessEqual,
            ">" => CompareOperator::GreaterThan,
            ">=" => CompareOperator::GreaterEqual, 
            _ => break
        };

        advance(position);

        let right = parse_factor(tokens, position)?;
        expr = Expression::CompareOp(Box::new(expr), operator, Box::new(right));
    }

    Ok(expr)
}

pub fn parse_boolean(tokens: &[Token], position: &mut usize) -> Result<Expression, ParseError> {
    let mut expr = parse_add_sub(tokens, position)?;

    while let Token::Operator(op) = current_token(tokens, *position) {
        let operator = match op.as_str() {
            "&&" => CompareOperator::And,
            "||" => CompareOperator::Or,
            _ => break,
        };

        advance(position);

        let right = parse_add_sub(tokens, position)?;
        expr = Expression::CompareOp(Box::new(expr), operator, Box::new(right));
    }

    Ok(expr)
}

fn parse_function_def(tokens: &[Token], position: &mut usize) -> Result<Stmt, ParseError> {
    if let Token::Def = current_token(tokens, *position) {
        advance(position);
    } else {
        return Err(ParseError::UnexpectedToken(format!("{:?}", current_token(tokens, *position))));
    }

    let function_name = if let Token::Identifier(name) = current_token(tokens, *position) {
        name.clone()
    } else {
        return Err(ParseError::InvalidIdentifier);
    };
    advance(position);

    if let Token::OpenParen = current_token(tokens, *postion) {
        advance(position);
    } else {
        return Err(ParseError::UnexpectedToken("Expected '('".to_string()));
    }

    if let Token::CloseParen = current_token(tokens, *position) {
        advance(position);
    } else {
        return Err(ParseError::UnexpectedToken("Expected ')'".to_string()));
    }

    let mut body = Vec::new();
    while !matches!(current_token(tokens, *position), Token::EOF) {
        body.push(parse_statement(tokens, position)?);
    }

    Ok(Stmt::FunctionDef(func_name, params, body))
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