mod farlib;

use std::{env, fs};
use std::fs::File;

fn print_help(args : Vec<String>) {
    println!("Usage: {} <command> <archive name> [FILES...]", args[0]);
    println!("Commands:");
    println!("  help - show this help message");
    println!("  test - test if file is a valid archive");
    println!("  list - list files in archive");
    return;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        print_help(args);
        return;
    }
    let command = &args[1];
    let archive_name = &args[2];
    match command.as_ref() {
        "help" => {
            print_help(args);
        },
        "test" => {
            let mut file = fs::read(archive_name).expect("Failed to read file ");
            let test = farlib::test(file);
            match test {
                Ok(archive_obj) => {
                    println!("{} is a valid archive", archive_name);
                    println!("Archive version: {}", archive_obj.version);
                },
                Err(e) => {
                    println!("{} is not a valid archive: {}", archive_name, e);
                }
            }
        },
        _ => {
            println!("Unknown command: {}", command);
        }
    }
}
