#[derive(Debug, PartialEq)]
pub enum Token {
    Number(i64),
    Identifier(String),
    Operator(String),
    OpenParen,
    CloseParen,
    Assign,
    Equal,
    EOF, //End of Input
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            // Skip Whitespace
            ' ' | '\t' | '\n' => {
                chars.next();
            }

            // Handle numbers
            '0'..='9' => {
                let mut number = String::new();
                while let Some(&digit) = chars.peek() {
                    if digit.is_numeric() {
                        number.push(digit);
                        chars.next();
                    } else {
                        break;
                    }
                }

                let number_value = number.parse::<i64>().unwrap();
                tokens.push(Token::Number(number_value));
            }

            // Handle identifiers (variables)
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut identifier = String::new();
                while let Some(&char) = chars.peek() {
                    if char.is_alphanumeric() || char == '_' {
                        identifier.push(char);
                        chars.next();
                    } else {
                        break;
                    }
                }

                tokens.push(Token::Identifier(identifier));
            }

            // Handle Operators
            '+' | '-' | '*' | '/' | '%' => {
                let operator =  ch.to_string();
                tokens.push(Token::Operator(operator));
                chars.next();
            }

            // Handle Assign
            '=' => {
                chars.next();
                if let Some(&'=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::Equal);
                } else {
                    tokens.push(Token::Assign);
                }
            }

            //Handle Parentheses
            '(' => {
                tokens.push(Token::OpenParen);
                chars.next();
            }

            ')' => {
                tokens.push(Token::CloseParen);
                chars.next();
            }

            _ => {
                chars.next();
            }
        }
    }

    tokens.push(Token::EOF);

    tokens
}