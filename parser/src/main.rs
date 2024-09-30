use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, multispace0},
    combinator::map,
    multi::many0,
    sequence::{pair, terminated, preceded},
    IResult,
};


#[derive(Debug)]
pub enum Expr {
    Number(i64),
    String(String),
    Variable(String),
    BinaryOp(Box<Expr>, BinaryOperator, Box<Expr>),
}

#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Subtract, 
    Multiply,
    Divide
}

#[derive(Debug)]
pub enum Stmt {
    Assignment(String, Expr)
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

fn main() {
    let input = "x = 1 + 2 * 3\ny = 4 - 5";

    match parse_program(input) {

        Ok((_, program)) => {
            println!("{:#?}", program);
        }

        Err(e) => {
            println!("Error parsing: {}", e)
        }
    }
}

fn parse_number(input: &str) -> IResult<&str, Expr> {
    map(
        preceded(multispace0, digit1),
         |n: &str| Expr::Number(n.parse().unwrap()))(input)
}

fn parse_identifier(input: &str) -> IResult<&str, Expr> {
    map(
        preceded(multispace0, alpha1),
         |var: &str| Expr::Variable(var.to_string()))(input)
}

fn parse_operator(input: &str) -> IResult<&str, BinaryOperator> {
    alt((
        map(preceded(multispace0, tag("+")), |_| BinaryOperator::Add),
        map(preceded(multispace0, tag("-")), |_| BinaryOperator::Subtract),
        map(preceded(multispace0,tag("*")), |_| BinaryOperator::Multiply),
        map(preceded(multispace0, tag("/")), |_| BinaryOperator::Divide),
    ))(input)
}

fn parse_expression(input: &str) -> IResult<&str, Expr> {

    let (input, init) = alt((parse_number, parse_identifier))(input)?;
    let (input, result) = many0(pair(parse_operator, alt((parse_number, parse_identifier))))(input)?;

    let expr = result. into_iter().fold(init, |acc, (op, right)| {
        Expr::BinaryOp(Box::new(acc), op, Box::new(right))
    });

    Ok((input, expr))
}

fn parse_assignment(input: &str) -> IResult<&str, Stmt> {
    
    let (input, var) = terminated(alpha1, multispace0)(input)?;
    let (input, _) = preceded(multispace0,tag("="))(input)?;
    let (input, expr) = parse_expression(input)?;
    
    Ok((input, Stmt::Assignment(var.to_string(), expr)))
}

fn parse_program(input: &str) -> IResult<&str, Program> {

    let (input, statements) = many0(terminated(parse_assignment, multispace0))(input)?;    

    Ok((input, Program { statements }))
}


