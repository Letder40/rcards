use std::io::stdout;
use crossterm::{cursor::{EnableBlinking, MoveTo, SavePosition}, execute, style::Color, terminal::{self, ClearType}};

use rcards::utils::console::*;
use rcards::utils::print::{print_colored, print_decks};
use rcards::utils::utils::get_decks;
use rcards::commands;

fn main_console() {
    ctrlc::set_handler(|| exit_console()).expect("error unsetting ctrl_c handler");
    loop {
        let user_input = console_prompt();
        let args: Vec<&str> = user_input.trim().split(" ").collect();
        match args[0] {
            "clear" => commands::clear(),
            "list" | "ls"  => commands::list(),
            "quit" | "exit" | "q" => exit_console(),
            "s" | "sel" => commands::select(args),
            "rm" => commands::remove(args),
            "n" | "new"  => commands::new(args),
            "a" | "add" => commands::add(args),
            "c" | "check" => commands::check(args),
            "?" | "help" => commands::help(),
            _ => {
                print_colored("[!] Error: ", Color::Red);
                print!("Invalid command, use ");
                print_colored("help ", Color::Green);
                println!("to see a list valids commands");
            }
        }
    }
}

fn main() {
    execute!(stdout(), SavePosition, MoveTo(0,0), EnableBlinking).unwrap();
    execute!(stdout(), terminal::Clear(ClearType::All)).unwrap();

    print_colored("Decks:\n", Color::Cyan);
    let decks = get_decks();  
    print_decks(&decks);

    main_console();
}
