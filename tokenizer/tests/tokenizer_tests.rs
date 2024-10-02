use tokenizer::*;

#[test]
fn test_tokenizer() {
    let input = "x = 5 + 3 * (2 - 8)";

    let expected_tokens = vec![
        Token::Identifier("x".to_string()),
        Token::Assign,
        Token::Number(5),
        Token::Operator("+".to_string()),
        Token::Number(3),
        Token::Operator("*".to_string()),
        Token::OpenParen,
        Token::Number(2),
        Token::Operator("-".to_string()),
        Token::Number(8),
        Token::CloseParen,
        Token::EOF,
    ];

    let tokens = tokenize(input);

    assert_eq!(tokens, expected_tokens);
}

#[test]
fn comparison() {
    let input = "y == 1";

    let  expected_tokens = vec![
        Token::Identifier("y".to_string()),
        Token::Equal,
        Token::Number(1),
        Token::EOF
    ];

    let tokens = tokenize(input);
    
    assert_eq!(tokens, expected_tokens);
}