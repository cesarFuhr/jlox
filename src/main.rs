use std::{
    //env,
    fs::File,
    io::{self, Read, Write},
    process::exit,
};

mod tree_walker;

fn main() {
    run_file("./test_script.lox".to_string())
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
