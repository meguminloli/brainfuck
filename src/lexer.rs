use std::{collections::VecDeque, io::Read, process::exit};
use std::io::Write;
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Loop {
    pub(crate) start: usize,
    pub(crate) end: usize,
}
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum Token {
    MoveLeft,
    MoveRight,
    Increment,
    Decrement,
    Output,
    Input,
    JumpLeft(Loop),
    JumpRight(Loop),
}

pub fn tokenize(s: String) -> Vec<Token>{
    let mut code = vec![Token::MoveRight; s.len()];
    let mut starts: VecDeque<usize> = VecDeque::new();
    let mut ends: VecDeque<usize> = VecDeque::new();
    for i in 0..s.len() {
        let ch = s.chars().nth(i).unwrap();
        if ch == '>' {
            code[i] = Token::MoveRight;
        } else if ch == '<' {
            code[i] = Token::MoveLeft;
        } else if ch == '+' {
            code[i] = Token::Increment;
        } else if ch == '-' {
            code[i] = Token::Decrement;
        } else if ch == '.' {
            code[i] = Token::Output;
        } else if ch == ',' {
            code[i] = Token::Input;
        } else if ch == '[' {
            if starts.is_empty() {
                starts.push_front(i);
                let mut l = 1;
                for j in i + 1..s.len() {
                    let c = s.chars().nth(j).unwrap();
                    if c == '[' {
                        l += 1;
                        starts.push_back(j)
                    } else if c == ']' {
                        l -= 1;
                        ends.push_front(j);
                    }
                    if l == 0 {
                        break;
                    }
                }
                if l != 0 {
                    println!("The code is wrong.");
                    exit(1);
                }
            }
            while !starts.is_empty() {
                let start = starts.pop_front().unwrap();
                let end = ends.pop_front().unwrap();
                let l = Loop {
                    start,
                    end
                };
                code[start] = Token::JumpLeft(l);
                code[end] = Token::JumpRight(l);
            }
        }
    }
    code
}

pub fn transpile_to_c(tokens: Vec<Token>, output: String) {
    let mut f = std::fs::File::create(output).unwrap();
    writeln!(&mut f, "#include <stdio.h>\n").unwrap();
    writeln!(&mut f, "int main () {{").unwrap();
    writeln!(&mut f, "    char arr[100000] = {{0}};").unwrap();
    writeln!(&mut f, "    char *p = arr+50000;").unwrap();
    let mut no_spaces = 4;
    for token in tokens {
        let mut s = "";
        let mut start_loop = false;
        match token {
            Token::MoveLeft => s = "--p;",
            Token::MoveRight => s = "++p;",
            Token::Increment => s = "++(*p);",
            Token::Decrement => s = "--(*p);",
            Token::Output => s = "putchar(*p);",
            Token::Input => s = "*p = getchar();",
            Token::JumpLeft(_) => {
                start_loop = true;
                s = "while (*p) {";
                no_spaces += 4;
            },
            Token::JumpRight(_) => {
                no_spaces -= 4;
                s = "}";
            },
        }
        if start_loop {
            writeln!(&mut f, "{}", beautify(s, no_spaces - 4)).unwrap();
        } else {
            writeln!(&mut f, "{}", beautify(s, no_spaces)).unwrap();
        }
    }
    writeln!(&mut f, "}}").unwrap();
}

pub fn execute_code(v: &Vec<Token>) {
    let mut p: usize = 50000;
    let mut i = 0;
    let mut buf = vec![0; 100000];
    while i < v.len() {
        let token = v[i];
        match token {
            Token::MoveLeft => p -= 1,
            Token::MoveRight => p += 1,
            Token::Increment => buf[p] += 1,
            Token::Decrement => buf[p] -= 1,
            Token::Input => {
                let mut buffer = [0; 1];
                std::io::stdin().read(&mut buffer[..]).unwrap();
                buf[p] = buffer[0] as u8;
            }
            Token::Output => print!("{}", buf[p] as u8 as char),
            Token::JumpLeft(l) => {
                if buf[p] == 0 {
                    i = l.end;
                }
            },
            Token::JumpRight(l) => {
                i = l.start - 1;
            }
        }
        i += 1;
    }
}

fn beautify(s: &str, no_spaces: usize) -> String {
    let v = vec![' ' as u8; no_spaces];
    let mut out = String::from_utf8(v).unwrap();
    out.push_str(s);
    out
}