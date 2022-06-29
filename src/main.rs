use colored::*;
use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio};
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

        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next() {
            let mut parts = command.trim().split_whitespace();
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
                    let stdin = previous_command.map_or(Stdio::inherit(), |output: Child| {
                        Stdio::from(output.stdout.unwrap())
                    });

                    let stdout = if commands.peek().is_some() {
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };

                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
                        Ok(output) => {
                            previous_command = Some(output);
                        }
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        }
                    };
                }
            }
        }
        if let Some(mut final_command) = previous_command {
            #[allow(unused_must_use)]
            {
                final_command.wait();
            }
        }
    }
}
