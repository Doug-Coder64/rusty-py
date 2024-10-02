use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, multispace0},
    combinator::{map, opt},
    multi::many0,
    sequence::{delimited, pair, terminated, preceded},
    IResult,
};

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

    //checks to see if the number is negative 
    let (input, sign) =  opt(preceded(multispace0, tag("-")))(input)?;
    let (input, num) = preceded(multispace0, digit1)(input)?;

    let value: i64 = num.parse().unwrap();
    let value = if sign.is_some() { -value } else { value };

    Ok((input, Expression::Number(value)))
}

// Parse Variables (identifiers)
fn parse_identifier(input: &str) -> IResult<&str, Expression> {
    map(
        preceded(multispace0, alpha1),
         |var: &str| Expression::Variable(var.to_string()))(input)
}

fn parse_operator(level: PrecedenceLevel, input: &str) -> IResult<&str, BinaryOperator> {

    match level {
        PrecedenceLevel::AddSub => alt((
            map(preceded(multispace0, tag("+")), |_| BinaryOperator::Add),
            map(preceded(multispace0, tag("-")), |_| BinaryOperator::Subtract)
        ))(input),
        PrecedenceLevel::Power => alt((
            map(preceded(multispace0, tag("**")), |_| BinaryOperator::Power),
        ))(input),
        PrecedenceLevel::MulDiv => alt((
            map(preceded(multispace0, tag("*")), |_| BinaryOperator::Multiply),
            map(preceded(multispace0, tag("//")), |_| BinaryOperator::FloorDivide), // Handle floor division
            map(preceded(multispace0, tag("/")), |_| BinaryOperator::Divide),
            map(preceded(multispace0, tag("%")), |_| BinaryOperator::Modulus),
        ))(input),
    }    
}

fn parse_power(input: &str) -> IResult<&str, Expression> {
    let (input, init) = alt((parse_number, parse_identifier, parse_parenthesize))(input)?;

    let (input, result) = many0(
        pair(
            |i| parse_operator(PrecedenceLevel::Power, i),
    alt((parse_number, parse_identifier, parse_parenthesize))
        )
    )(input)?;

    let expr = result.into_iter().fold(init, |acc, (op, right)| {
        Expression::BinaryOp(Box::new(acc), op, Box::new(right))
    }); 

    Ok((input, expr))
}

// parse for factors
fn parse_factor(input: &str) -> IResult<&str, Expression> {
    
    let (input, init) = parse_power(input)?;
    let (input, result) = many0(
            pair(
                |i| parse_operator(PrecedenceLevel::MulDiv, i),
        parse_power
            )
        )(input)?;

    let expr = result.into_iter().fold(init, |acc, (op, right)| {
        Expression::BinaryOp(Box::new(acc), op, Box::new(right))
    });

    Ok((input, expr))
}


//handles expressions wrapped in parentheses 
fn parse_parenthesize(input: &str) -> IResult<&str, Expression> {
    delimited(
        preceded(multispace0, tag("(")),
        parse_expression,
        preceded(multispace0, tag(")"))
    )(input)
}


fn parse_expression(input: &str) -> IResult<&str, Expression> {

    let (input, init) = parse_factor(input)?;
    let (input, result) = many0(
            pair(
            |i| parse_operator(PrecedenceLevel::AddSub, i),
            parse_factor
            )
        )(input)?;

    let expr = result. into_iter().fold(init, |acc, (op, right)| {
        Expression::BinaryOp(Box::new(acc), op, Box::new(right))
    });

    Ok((input, expr))
}

fn parse_assignment(input: &str) -> IResult<&str, Stmt> {
    
    let (input, var) = terminated(alpha1, multispace0)(input)?;
    let (input, _) = preceded(multispace0,tag("="))(input)?;
    let (input, expr) = parse_expression(input)?;
    
    Ok((input, Stmt::Assignment(var.to_string(), expr)))
}

pub fn parse_program(input: &str) -> IResult<&str, Program> {

    let (input, statements) = many0(terminated(parse_assignment, multispace0))(input)?;    

    Ok((input, Program { statements }))
}