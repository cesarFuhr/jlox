use std::{
    //env,
    fs::File,
    io::{self, Read, Write},
    process::exit,
};

use self::tree_walker::{
    syntax_tree::{Binary, Expr, Grouping, Literal, PrettyPrint, Unary},
    tokens::{LiteralType, Token, TokenType},
};

mod tree_walker;

fn main() {
    // let mut args = env::args().skip(1);

    let left = Expr::Unary(Box::new(Unary::new(
        Token {
            r#type: TokenType::Minus,
            lexeme: String::from("-"),
            line: 1,
            literal: None,
        },
        Expr::Literal(Literal::new(LiteralType::Number(123.0))),
    )));

    let op = Token {
        r#type: TokenType::Star,
        lexeme: String::from("*"),
        line: 1,
        literal: None,
    };

    let right = Expr::Grouping(Box::new(Grouping::new(Expr::Literal(Literal::new(
        LiteralType::Number(45.67),
    )))));

    let e = Expr::Binary(Box::new(Binary::new(left, op, right)));

    println!("{}", e.pretty_print());

    // match args.len() {
    //     0 => run_prompt(),
    //     1 => run_file(args.next().take().unwrap()),
    //     _ => {
    //         println!("Usage: jlox [script]");
    //         exit(64)
    //     }
    // }
}

fn run_file(file_path: String) {
    let source = match File::open(file_path) {
        Err(error) => {
            println!("{}", error);
            exit(1)
        }
        Ok(mut file) => {
            let mut buf = String::new();
            let _ = file.read_to_string(&mut buf).unwrap_or_else(|err| {
                println!("{}", err);
                exit(1);
            });

            buf
        }
    };

    if let Err(e) = run(source) {
        tree_walker::errors::report(e);
        exit(65);
    }
}

fn run_prompt() {
    print!("> ");
    let mut buf = String::new();
    loop {
        io::stdout().flush().unwrap();

        let num_bytes = io::stdin().read_line(&mut buf).unwrap_or_else(|err| {
            println!("{}", err);
            exit(1);
        });

        if num_bytes == 0 {
            break;
        }

        if let Err(e) = run(buf.to_string()) {
            tree_walker::errors::report(e);
        }
        buf.clear();

        print!("> ");
    }
}

fn run(source: String) -> Result<(), tree_walker::errors::Error> {
    let mut scanner = tree_walker::scanner::Scanner::new(source);
    let tokens = scanner.scan_tokens();

    println!("{:?}", tokens);
    Ok(())
}
