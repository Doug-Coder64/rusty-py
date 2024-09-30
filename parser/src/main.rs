use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, multispace0},
    combinator::map,
    multi::many0,
    sequence::{pair, terminated, preceded},
    IResult,
};


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
    Divide
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Assignment(String, Expression)
}

#[derive(Debug, PartialEq)]
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

fn parse_number(input: &str) -> IResult<&str, Expression> {
    map(
        preceded(multispace0, digit1),
         |n: &str| Expression::Number(n.parse().unwrap()))(input)
}

// Parse Variables (identifiers)
fn parse_identifier(input: &str) -> IResult<&str, Expression> {
    map(
        preceded(multispace0, alpha1),
         |var: &str| Expression::Variable(var.to_string()))(input)
}


fn parse_operator(input: &str) -> IResult<&str, BinaryOperator> {
    alt((
        map(preceded(multispace0, tag("+")), |_| BinaryOperator::Add),
        map(preceded(multispace0, tag("-")), |_| BinaryOperator::Subtract),
        map(preceded(multispace0,tag("*")), |_| BinaryOperator::Multiply),
        map(preceded(multispace0, tag("/")), |_| BinaryOperator::Divide),
    ))(input)
}

// parse for factors
fn parse_factor(input: &str) -> IResult<&str, Expression> {
    
    let (input, init) = alt((parse_number, parse_identifier))(input)?;
    let (input, result) = many0(pair(
        alt((
            map(preceded(multispace0, tag("*")), |_| BinaryOperator::Multiply),
            map(preceded(multispace0, tag("/")), |_| BinaryOperator::Divide)
        )),
        alt((parse_number, parse_identifier))
    ))(input)?;

    let expr = result.into_iter().fold(init, |acc, (op, right)| {
        Expression::BinaryOp(Box::new(acc), op, Box::new(right))
    });

    Ok((input, expr))
}


fn parse_expression(input: &str) -> IResult<&str, Expression> {

    let (input, init) = parse_factor(input)?;
    let (input, result) = many0(pair(
        alt((
            map(preceded(multispace0, tag("+")), |_| BinaryOperator::Add),
            map(preceded(multispace0, tag("-")), |_| BinaryOperator::Subtract)
        )),
        parse_factor
        ))(input)?;

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

fn parse_program(input: &str) -> IResult<&str, Program> {

    let (input, statements) = many0(terminated(parse_assignment, multispace0))(input)?;    

    Ok((input, Program { statements }))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests order of operations and multiple lines/Assignments
    #[test]
    fn multi_line_program() {
        let input = "x= 1 + 2 * 3\ny = 4 - 5";

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
                            Box::new(Expression::Number(3))
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
            ],
        };

        match parse_program(input) {
            Ok((_, program)) => {
                assert_eq!(program, expected);
            }

            Err(e) => panic!("Failed to parse program: {:?}", e),
        }
    }

    
}
