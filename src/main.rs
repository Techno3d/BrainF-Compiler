use std::env;
use std::process;
use std::fs;

//Commands in brainf
#[derive(Debug, PartialEq, Copy, Clone)]
enum Token {
    Add,        // +
    Subtract,   // -
    Right,      // >
    Left,       // <
    Read,       // ,
    Write,      // .
    BeginLoop,  // [
    EndLoop,    // ]
}

fn main() {
    println!("{:?}", env::args());
    if env::args().len() < 2 {
        println!("Give a brainf file to compile");
        process::exit(1);
    }
    
    let mut file = env::args().last().expect("No File Given");
    let contents = fs::read_to_string(&file).expect("Couldn't read it sad");
    
    let tokens = tokenize(&contents);
    println!("{:?}", tokens);

    let generated_code = generate_c(&tokens);
    file.push_str(".c");
    fs::write(file, generated_code).expect("Lmao");
    
    
}

fn tokenize(command: &str) -> Vec<Token> {
    let mut tokens = Vec::<Token>::new();

    let chars = command.chars();

    for c in chars.into_iter() {
        match c {
            '+' => tokens.push(Token::Add),
            '-' => tokens.push(Token::Subtract),
            '>' => tokens.push(Token::Right),
            '<' => tokens.push(Token::Left),
            ',' => tokens.push(Token::Read),
            '.' => tokens.push(Token::Write),
            '[' => tokens.push(Token::BeginLoop),
            ']' => tokens.push(Token::EndLoop),
            _ => {}
        }
    }

    tokens
}


fn generate_c(tokens: &[Token]) -> String {
    let mut output = String::from(r#"
        #include "stdio.h"
        
        int main()
        {
            char tape[20000] = {0};
            char *ptr = tape;

    "#);

    for &token in tokens {
        match token {
            Token::Add => {
                output.push_str("\t\t++*ptr;\n");
            },
            Token::Subtract => {
                output.push_str("\t\t--*ptr;\n");
            },
            Token::Right => {
                output.push_str("\t\t++ptr;\n");
            },
            Token::Left => {
                output.push_str("\t\t--ptr;\n");
            },
            Token::Read => {
                output.push_str("\t\t*ptr = getchar();\n");
            },
            Token::Write => {
                output.push_str("\t\tputchar(*ptr);\n");
            },
            Token::BeginLoop => {
                output.push_str("\twhile (*ptr) {;\n");
            },
            Token::EndLoop => {
                output.push_str("\t};\n");
            },
        }
    }
    
    output.push_str("}\n");

    output
}