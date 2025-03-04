use std::{ffi::OsString, fs::{self}, io::stdout, path::Path};
use crossterm::{cursor::{EnableBlinking, MoveTo, SavePosition}, execute, style::Color, terminal::{self, ClearType}};
use tabled::settings::{object::{FirstRow, Segment}, Alignment, Margin, Modify, Padding, Style, Width};

use crate::{deck::{Card, Deck, DeckError, DeckSaveMode}, utils::utils::*};
use crate::utils::print::*;

use crate::utils::console::input_prompt;

pub fn clear() {
    execute!(stdout(), terminal::Clear(ClearType::All)).unwrap();
    execute!(stdout(), SavePosition, MoveTo(0,0), EnableBlinking).unwrap();
}

pub fn list() {
    let decks = &get_decks();
    print_decks(decks);
}

pub fn select(args: Vec<&str>) {
    if args.len() != 2 {
        print_colored("[!] Error: ", Color::Red);
        println!("Ivalid deck name");
        return;
    }

    let selected_card = args[1..].to_vec().join(" ");
    let path = match get_deckpath(&selected_card) {
        Ok(path) => path,
        Err(_) => {
            return;
        },
    };

    let deck: Deck = match Deck::from_path(path) {
        Ok(deck) => deck,
        Err(err) => {
            match err {
                DeckError::Io(_) => print_flag(Flags::Error, format!("{selected_card} does not exists")),
                DeckError::Serde(_) => print_flag(Flags::Error, format!("{selected_card} is not a valid rcard: json formating error")),
                
            }
            return;
        }
    };

    print_colored("q ", Color::Green);
    println!("to exit\n");

    print_colored("[#] ", Color::Cyan);
    println!("Deck: {}\n", deck.name);

    for (index, card) in deck.cards.iter().enumerate() {
        print_colored("Card: ", Color::Blue);
        print_colored(index.to_string().as_str(), Color::Green);
        println!();
        print_colored("Front: ", Color::Blue);
        println!("{}", card.front);
        let user_input = input_prompt().to_lowercase();

        if user_input == "q" {
            break; 
        }
        
        let user_input_chars: Vec<char> = user_input.to_lowercase().chars().collect();
        let chars_count = card.back.chars().count();
        let mut matches: u128 = 0;
        for (index, char) in card.back.chars().enumerate() {
            let bitmask = 1 << index;
            if index < user_input.len() && char == user_input_chars[index] {
                matches = matches | bitmask;
            }
        }

        let mut matches_counter = 0;
        for char in card.back.chars() {
            let bit = matches & 1;
            if bit & 1 == 1 {
                print_colored(char.to_string().as_str(), Color::Green);
                matches_counter += 1;
            } else {
                print_colored(char.to_string().as_str(), Color::Red);
            }
            matches = matches >> 1;
        }
        println!();
        let accuaracy = matches_counter as f32 / chars_count as f32 * 100.0; 
        print_colored("Accuaracy: ", Color::Blue);
        print_colored(&format!("{accuaracy}%\n\n").to_string(), Color::Green);
    }
}

pub fn remove(args: Vec<&str>) {
    let card_id_name = args[1..].join(" ");
    let path = match get_deckpath(&card_id_name) {
        Ok(path) => path,
        Err(_) => {
            return;
        },
    };

    match fs::remove_file(path) {
        Ok(_) => {},
        Err(_) => {
            print_flag(Flags::Error, "Deck not exists");
            return;
        }
    };
}

enum Action {
    Exit,
    Continue,
}

fn handle_action(input: &String, name: &String, cards: &Vec<Card>, path: &Path, deck_operation: &DeckSaveMode) -> Action {
    match input.as_str() {
        "w" => {
            match Deck::new(name.to_owned(), cards.clone()).save(path, deck_operation) {
                Ok(_) => {},
                Err(_) => print_flag(Flags::Error, "unexpected error while saving desk data"),
            };
            Action::Exit
        },
        "q" => {
            Action::Exit
        },
        _ => {
            Action::Continue
        },
    }
}

fn new_cards(mut cards: Vec<Card>, deck_name: String, path: &Path, deck_save_mode: DeckSaveMode) {
    print_flag(Flags::Info, "Help: ");
    print_colored("q ", Color::Red);
    println!("to exit");
    print_colored("w ", Color::Green);
    println!("to save and exit\n");

    let mut n = 0;
    loop {
        n += 1;
        print_flag(Flags::Info, format!("front: {n}"));
        let front = input_prompt();
        match handle_action(&front, &deck_name, &cards, path, &deck_save_mode) {
            Action::Continue => {},
            Action::Exit => return,
        }

        print_flag(Flags::Info, "Reverse: ");
        let back = input_prompt();
        match handle_action(&back, &deck_name, &cards, path, &deck_save_mode) {
            Action::Continue => {},
            Action::Exit => return,
        }
        
        let rcard: Card = Card { 
            front: front.
                trim()
                .to_owned(),
            back: back.to_string()
                .trim()
                .to_owned()
                .to_lowercase()
        };
        cards.push(rcard);

        println!();
    }
}

pub fn new(args: Vec<&str>) {
    let deck_name = args[1..].join(" ");
    let deck_name_ostr: OsString = match deck_name.clone().try_into() {
        Ok(ostr) => ostr,
        Err(_) => {
            print_flag(Flags::Error, "Invalid deck name");
            return;
        }
    };

    let mut path = std::env::var_os("HOME").expect("home env var is not set, HOME env variable is used to determine card vault");
    path.push("/.rcard_vault/");
    path.push(&deck_name_ostr);
    path.push(".rcard");

    let path: &Path = Path::new(&path);
    let cards: Vec<Card> = Vec::new();
    let deck_save_mode: DeckSaveMode = DeckSaveMode::New;
    new_cards(cards, deck_name, path, deck_save_mode);
}

pub fn add(args: Vec<&str>) {
    let card_id_name = args[1..].join(" ");
    let path = match get_deckpath(&card_id_name) {
        Ok(path) => path,
        Err(_) => {
            return;
        },
    };

    let deck = match Deck::from_path(&path) {
        Ok(deck) => deck,
        Err(err) => {
            match err {
                DeckError::Io(_) => print_flag(Flags::Error, format!("{card_id_name} does not exists")),
                DeckError::Serde(_) => print_flag(Flags::Error, format!("{card_id_name} is not a valid rcard: json formating error")),
                
            }
            return;
        }
    };

    let deck_save_mode: DeckSaveMode = DeckSaveMode::Add;
    new_cards(deck.cards, deck.name, path.as_path(), deck_save_mode);
}

pub fn check(args: Vec<&str>) {
    let card_id_name = args[1..].join(" ");
    let path = match get_deckpath(&card_id_name) {
        Ok(path) => path,
        Err(_) => {
            return;
        },
    };

    let deck = match Deck::from_path(&path) {
        Ok(deck) => deck,
        Err(err) => {
            match err {
                DeckError::Io(_) => print_flag(Flags::Error, format!("{card_id_name} does not exists")),
                DeckError::Serde(_) => print_flag(Flags::Error, format!("{card_id_name} is not a valid rcard: json formating error")),
                
            }
            return;
        }
    };
    
    let mut table_builder = tabled::builder::Builder::default();

    table_builder.push_record(vec!["Front", "Back"]);
    for card in deck.cards {
        table_builder.push_record(vec![card.front, card.back]);
    }

    let mut table = table_builder.build();
    table.with(Style::modern_rounded());
    table.with(Padding::new(1, 1, 0, 0));
    table.modify(FirstRow, Alignment::center());
    table.with(Margin::new(2, 0, 1, 1));
    table.with(Modify::new(Segment::all()).with(Width::wrap(30)));
    
    println!("{table}")
}

pub fn help() {
    let commands = vec!["ls", "sel", "check", "add", "new", "rm", "check", "quit"];

    let mut table_builder = tabled::builder::Builder::default();
    table_builder.push_record(vec!["Commands"]);
    for command in commands {
        table_builder.push_record(vec![command]);
    }

    let mut table = table_builder.build();
    table.with(Style::modern_rounded());
    table.with(Padding::new(1, 1, 0, 0));
    table.modify(FirstRow, Alignment::center());
    table.with(Margin::new(2, 0, 1, 1));
    table.with(Modify::new(Segment::all()).with(Width::wrap(30)));
    
    println!("{table}")
}
