mod lexer;
mod utilities;
use lexer::*;
use utilities::*;

fn main() {
    if std::env::args().len() < 2 {
        println!("Usage:\nbrainfuck <input> <output>\nOutput is a optional argument if you want to transpile to C.");
        return;
    }
    let s = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let mut o = None;
    if std::env::args().len() == 3 {
        o = Some(std::env::args().nth(2).unwrap());
    }
    let s = clear_string(s);
    let code = tokenize(s);
    match o {
        Some(s) => transpile_to_c(code, s),
        _ => execute_code(&code),
    }
}
