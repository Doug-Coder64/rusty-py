use parser::parse_program;
use tokenizer::tokenize;
fn main() {
    let input = "x = False\n y = 1 + 2 - 3 * 4\nz = (1 < 2)";//"x = 1 / 2 ** -3\ny = 4 // 5 ** 2\nz = (2 - 1) + 1";

    let tokens = tokenize(input);
    println!("Tokens: {:#?}",  tokens);
    
    let program = parse_program(&tokens);
    println!("Parsed Program: {:#?}", program);

}




