use colored::*;
use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::Command;
#[allow(unused_imports)]
use whoami::*;

fn main() {
    let at = "@";
    let twenty = "--------------------";
    loop {
        print!(
            "{}{}{}->",
            whoami::username().magenta(),
            at.yellow(),
            whoami::hostname().blue(),
        );
        #[allow(unused_must_use)]
        {
            stdout().flush();
        }

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "cd" => {
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            }
            "exit" => return,
            "pan" => {
                for _x in 0..5 {
                    println!("{}{}", twenty.magenta(), twenty.magenta());
                }
                for _x in 0..5 {
                    println!("{}{}", twenty.yellow(), twenty.yellow());
                }
                for _x in 0..5 {
                    println!("{}{}", twenty.blue(), twenty.blue());
                }
            }
            command => {
                let child = Command::new(command).args(args).spawn();

                match child {
                    Ok(mut child) => {
                        #[allow(unused_must_use)]
                        {
                            child.wait();
                        }
                    }
                    Err(e) => eprintln!("{}", e),
                };
            }
        }
    }
}
