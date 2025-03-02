use std::{fmt::Display, io::stdout};
use crossterm::{execute, style::{Color, Print, ResetColor, SetForegroundColor}};
use tabled::settings::{object::FirstRow, Alignment, Margin, Padding, Style};

pub fn print_colored(text: &str, color: Color) {
    execute!(
        stdout(),
        SetForegroundColor(color),
        Print(text),
        ResetColor,
    ).unwrap();
}

pub enum Flags {
    Error,
    Info,
}

pub fn print_flag(flag: Flags, text: impl Display) {
    match flag {
        Flags::Error => {
            print_colored("[#] ", Color::Red);
            print!("Error: ");
        } 
            
        Flags::Info => {
            print_colored("[#]", Color::Cyan);
        },

    }
    println!("{text}");
}

pub fn print_decks(decks: &Vec<String>) {
    if decks.len() == 0 {
        return;
    }
    let mut table_builder = tabled::builder::Builder::default();
    let header = vec!["ID", "Decks"];
    table_builder.push_record(header);

    for (id, deck) in decks.iter().enumerate() {
        table_builder.push_record(vec![id.to_string(), deck.to_owned()]);
    }

    let mut table = table_builder.build();
    table.with(Style::rounded());
    table.with(Padding::new(2, 2, 0, 0));
    table.modify(FirstRow, Alignment::center());
    table.with(Margin::new(2, 0, 1, 1));
    
    println!("{table}")
}
