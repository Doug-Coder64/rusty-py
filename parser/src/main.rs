use parser::parse_program;

fn main() {
    let input = "x = 1 % 2 * -3\ny = 4 // 5 / 2\nz = (2 - 1) + 1";

    match parse_program(input) {

        Ok((_, program)) => {
            println!("{:#?}", program);
        }

        Err(e) => {
            println!("Error parsing: {}", e)
        }
    }
}




