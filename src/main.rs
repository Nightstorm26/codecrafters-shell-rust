// #[allow(unused_imports)]
// use std::io::{self, Write};

// fn main() {
//     loop{
//         print!("$ ");
//         io::stdout().flush().unwrap();

//         //Wait for user input
//         let mut command = String::new();
//         io::stdin().read_line(&mut command).unwrap();

//         command = command.trim().to_string();
//         if command == "exit" {
//             break;
//         } else if command.starts_with("echo") {
//             println!("{}", &command[5..]);
//         } else if command.starts_with("type") {
            
//         }
//         else {
//             println!("{}: command not found", command);
//         }
//     }
// }
use std::io::Read;
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut command = String::new();

    loop {
        print!("$ ");
        io::stdout().flush()?;

        parse_command_str(&mut command)?;
        process_command(&command)?;

        command.clear();
    }
}

fn get_command(command: &mut String) -> io::Result<usize> {
    io::stdin().read_line(command)
}

fn parse_command_str(command: &mut String) -> io::Result<()> {
    get_command(command)?;

    Ok(())
}

fn process_command(command: &str) -> io::Result<()> {
    let _command = command.split_ascii_whitespace().collect::<Vec<_>>();
    match _command.as_slice() {
        ["exit", ..] => std::process::exit(0),
        ["echo", ..] => {
            let message = command
                .split_once(char::is_whitespace)
                .map(|(_, rest)| rest.trim())
                .unwrap_or_default();

            func_echo(message);
        }

        ["type", rest @ ("type" | "echo" | "exit"), ..] => println!("{} is a shell builtin", rest),
        ["type", rest @ ..] => println!("{}: not found", rest[0]),
        _ => println!("{}: command not found", _command[0]),
    }

    Ok(())
}

fn func_echo(message: &str) {
    println!("{}", message);
}
