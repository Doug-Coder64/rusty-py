use parser::parse_program;
use tokenizer::tokenize;
fn main() {
    let input = "x = 1 + 2 - 3 * 4 // 5 / 6 % 7 ** (8 + 9)";//"x = 1 / 2 ** -3\ny = 4 // 5 ** 2\nz = (2 - 1) + 1";

    let tokens = tokenize(input);
    println!("Tokens: {:#?}",  tokens);
    
    let program = parse_program(&tokens);
    println!("Parsed Program: {:#?}", program);

}




