use std::string;

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(i64),
    Identifier(String),
    Operator(String),
    String(String),
    Boolean(bool),
    OpenParen,
    CloseParen,
    Assign,
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

            //Handle boolean literals
            'T' | 'F' => {
                let mut literal = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() {
                        literal.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }

                if literal == "True" {
                    tokens.push(Token::Boolean(true));
                } else if literal == "False" {
                    tokens.push(Token::Boolean(false));
                } else {
                    tokens.push(Token::Identifier(literal));
                }
            }

            //Handle String
            '"' => {
                chars.next();
                let mut string_value = String::new();

                while let Some(&char) = chars.peek()  {
                    if char == '"' {
                        chars.next();
                        break;
                    } else {
                        string_value.push(char);
                        chars.next();
                    }
                }

                tokens.push(Token::String(string_value));
            }

            // Handle numbers and subtraction
            '0'..='9' | '-' => {
                let mut number = String::new();

                if ch == '-' {
                    chars.next();
                    if let Some('0'..='9') = chars.peek() {
                        number.push('-');
                    } else {
                        tokens.push(Token::Operator("-".to_string()));
                        continue;
                    }
                }

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

            // Handle additon and modulus
            '+' | '%' => {
                let operator =  ch.to_string();
                tokens.push(Token::Operator(operator));
                chars.next();
            }

            //Multiplication and Power
            '*' => {
                chars.next();
                if let Some(&'*') = chars.peek() {
                    chars.next();
                    tokens.push(Token::Operator("**".to_string()));
                } else {
                    tokens.push(Token::Operator("*".to_string()))
                }
            }

            //Division and floor division
            '/' => {
                chars.next();
                if let Some(&'/') = chars.peek() {
                    chars.next();
                    tokens.push(Token::Operator("//".to_string()));
                } else {
                    tokens.push(Token::Operator("/".to_string()))
                }
            }


            // Handle Assign and comparison
            '=' => {
                chars.next();
                if let Some(&'=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::Operator("==".to_string()));
                } else {
                    tokens.push(Token::Assign);
                }
            }

            '!' => {
                chars.next();
                if let Some('=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::Operator("!=".to_string()));
                }
            }

            '<' => {
                chars.next();
                if let Some('=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::Operator("<=".to_string()))
                } else {
                    tokens.push(Token::Operator("<".to_string()))
                }
            }

            '>' => {
                chars.next();
                if let Some('=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::Operator(">=".to_string()));
                } else {
                    tokens.push(Token::Operator(">".to_string()));
                }
            }

            '&' => {
                chars.next();
                if let Some('&') = chars.peek() {
                    chars.next();
                    tokens.push(Token::Operator("&&".to_string()));
                }
            }

            '|' => {
                chars.next();
                if let Some('|') = chars.peek() {
                    chars.next();
                    tokens.push(Token::Operator("||".to_string()));
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