use parser::parse_program;

fn main() {
    let input = "x = 1 + 2 - 3 * 4 / 5 // 6 % 7 ** (8 + 9)";//"x = 1 / 2 ** -3\ny = 4 // 5 ** 2\nz = (2 - 1) + 1";

    match parse_program(input) {

        Ok((_, program)) => {
            println!("{:#?}", program);
        }

        Err(e) => {
            println!("Error parsing: {}", e)
        }
    }
}




