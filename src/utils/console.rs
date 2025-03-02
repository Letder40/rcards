use std::{io::{stdout, Write}, process::exit};
use crossterm::{execute, style::Color, terminal::{self}};

use crate::utils::print::print_colored;

pub fn exit_console() {
    execute!(stdout(), terminal::LeaveAlternateScreen).unwrap();
    exit(0);
}

pub fn console_prompt() -> String {
    print_colored("[#] ", Color::Blue);
    print_colored("Deck Selector\n", Color::Yellow);
    stdout().flush().unwrap();

    let mut rl = rustyline::DefaultEditor::new().unwrap();
    let readline = rl.readline("\x1b[1;32m> \x1b[0m");
    
    match readline {
        Ok(line) => return line,
        Err(_) => return "".to_owned(),
    }
}

pub fn input_prompt() -> String {
    let mut rl = rustyline::DefaultEditor::new().unwrap();
    let readline = rl.readline("\x1b[1;32m> \x1b[0m");
    
    match readline {
        Ok(line) => return line,
        Err(_) => return "".to_owned(),
    }
}
