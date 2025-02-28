use std::{fs::{self, DirEntry, ReadDir}, io::{self, stdout, BufReader, Error, Result, Write}, path::{Path, PathBuf}, process::exit, time::SystemTime};
use serde::{Deserialize, Serialize};
use crossterm::{cursor::{EnableBlinking, MoveTo, SavePosition}, execute, style::{Color, Print, ResetColor, SetForegroundColor}, terminal::{self, ClearType}};
use tabled::settings::{object::FirstRow, Alignment, Margin, Padding, Style};

#[derive(Serialize, Deserialize)]
struct Card {
    front: String,
    back: String
}

#[derive(Serialize, Deserialize)]
struct Theme {
    name: String,
    cards: Vec<Card>
}

fn print_colored(text: &str, color: Color) {
    execute!(
        stdout(),
        SetForegroundColor(color),
        Print(text),
        ResetColor,
    ).unwrap();
}

fn read_rcard_vault() -> ReadDir {
    let mut path = std::env::var_os("HOME").expect("home env var is not set, HOME env variable is used to determine card vault");
    path.push("/.rcard_vault");
    let rcards_vault_path = Path::new(&path);
    if rcards_vault_path.exists() {
        fs::read_dir(rcards_vault_path).expect("Error while openning rcard_vault")
    } else {
        fs::create_dir(rcards_vault_path).expect("Vault path does not exists and creation failled");
        fs::read_dir(rcards_vault_path).expect("Error while openning rcard_vault")
    }
}

fn get_themes() -> Vec<String> {
    let entries: Vec<Result<DirEntry>> = read_rcard_vault().collect();

    let mut entries: Vec<(String, SystemTime)> = entries.iter()
        .filter_map(|entry| entry.as_ref().ok())
        .map(|entry| (entry.file_name().to_string_lossy().to_string(), entry.metadata().unwrap().accessed().unwrap()))
        .collect();

    entries.sort_by(|a, b| b.1.cmp(&a.1));

    entries.iter()
        .filter(|a| a.0.ends_with(".rcard"))
        .map(|a| a.0.strip_suffix(".rcard").unwrap().to_owned())
        .collect()
}

fn print_themes(themes: &Vec<String>) {
    let mut table_builder = tabled::builder::Builder::default();
    let header = vec!["ID", "Themes"];
    table_builder.push_record(header);

    for (id, theme) in themes.iter().enumerate() {
        table_builder.push_record(vec![id.to_string(), theme.to_owned()]);
    }

    let mut table = table_builder.build();
    table.with(Style::rounded());
    table.with(Padding::new(2, 2, 0, 0));
    table.modify(FirstRow, Alignment::center());
    table.with(Margin::new(2, 0, 1, 1));
    
    println!("{table}")
}

fn console_prompt() -> String {
    print_colored("[#] ", Color::Blue);
    print_colored("Theme Selector\n", Color::Yellow);
    stdout().flush().unwrap();
    let mut rl = rustyline::DefaultEditor::new().unwrap();
    let readline = rl.readline("\x1b[1;32m> \x1b[0m");
    
    match readline {
        Ok(line) => return line,
        Err(_) => return "".to_owned(),
    }
}

fn input_prompt() -> String {
    let mut rl = rustyline::DefaultEditor::new().unwrap();
    stdout().flush().unwrap();
    let readline = rl.readline("\x1b[1;32m> \x1b[0m");
    
    match readline {
        Ok(line) => return line,
        Err(_) => return "".to_owned(),
    }
}

fn exit_console() {
    execute!(stdout(), terminal::LeaveAlternateScreen).unwrap();
    exit(0);
}

fn get_theme_from_id(id: usize) -> Result<PathBuf> {
    let mut path = std::env::var_os("HOME").expect("home env var is not set, HOME env variable is used to determine card vault");
    let themes = get_themes();
    if themes.len() - 1 >= id {
        path.push("/.rcard_vault/");
        path.push(&themes[id]);
        path.push(".rcard");
    } else {
        print_colored("[!] Error: ", Color::Red);
        println!("Invalid Id");
        return Err(Error::new(io::ErrorKind::NotFound, "Id not found"));
    }

    let path = PathBuf::from(path);

    if !path.exists() {
        print_colored("[!] Error: ", Color::Red);
        println!("Invalid Id");
        return Err(Error::new(io::ErrorKind::NotFound, "File not found"));
    }

    Ok(path)
}

fn get_theme_from_name(name: &str) -> Result<PathBuf> {
    let mut path = std::env::var_os("HOME").expect("home env var is not set, HOME env variable is used to determine card vault");
    path.push("/.rcard_vault/");
    path.push(name);
    path.push(".rcard");

    let path = PathBuf::from(path);

    if !path.exists() {
        print_colored("[!] Error: ", Color::Red);
        println!("theme not exists");
        return Err(Error::new(io::ErrorKind::NotFound, "Id not found"));
    }

    Ok(path)
}

fn main_console() {
    loop {
        let user_input = console_prompt();
        let command: Vec<&str> = user_input.trim().split(" ").collect();
        match command[0] {
            "clear" => {
                execute!(stdout(), terminal::Clear(ClearType::All)).unwrap();
                execute!(stdout(), SavePosition, MoveTo(0,0), EnableBlinking).unwrap();
            }
            "sel" => {
                if command.len() != 2 {
                    print_colored("[!] Error: ", Color::Red);
                    println!("Ivalid theme name");
                    continue;
                }

                let path = if command[1].parse::<usize>().is_ok() {
                    let id: usize = command[1].parse().unwrap();
                    match get_theme_from_id(id) {
                        Ok(path) => path,
                        Err(_) => continue,
                    }
                } else {
                    match get_theme_from_name(command[1]) {
                        Ok(path) => path,
                        Err(_) => continue,
                    }
                };

                let theme_file = fs::File::open(path).unwrap();
                let reader = BufReader::new(theme_file);
                let theme_data: Theme = match serde_json::from_reader(reader) {
                    Ok(theme_file) => theme_file,
                    Err(_) => {
                        print_colored("[!] Error: ", Color::Red);
                        println!("Ivalid json format in theme file");
                        continue
                    }
                };

                print_colored("q ", Color::Green);
                println!("to exit\n");

                print_colored("[#] ", Color::Cyan);
                println!("Theme: {}\n", theme_data.name);

                for (index, card) in theme_data.cards.iter().enumerate() {
                    print_colored("Card: ", Color::Blue);
                    print_colored(index.to_string().as_str(), Color::Green);
                    println!();
                    print_colored("Front: ", Color::Blue);
                    println!("{}", card.front);
                    let user_input = input_prompt();
                    println!();

                    if user_input.trim() == "q" {
                        break; 
                    }
                }

            }
            "rm" => {
                if command.len() != 2 {
                    print_colored("[!] Error: ", Color::Red);
                    println!("Ivalid theme name");
                    continue;
                }

                let path = if command[1].parse::<usize>().is_ok() {
                    let id: usize = command[1].parse().unwrap();
                    match get_theme_from_id(id) {
                        Ok(path) => path,
                        Err(_) => continue,
                    }
                } else {
                    match get_theme_from_name(command[1]) {
                        Ok(path) => path,
                        Err(_) => continue,
                    }
                };

                match fs::remove_file(path) {
                    Ok(_) => {},
                    Err(_) => {
                        print_colored("[!] Error: ", Color::Red);
                        println!("Theme not exists");
                        continue;
                    }
                };
            }
            "new" | "add" => {
                if command.len() != 2 {
                    print_colored("[!] Error: ", Color::Red);
                    println!("Ivalid theme name");
                    continue;
                }

                if command[1].parse::<usize>().is_ok() {
                    print_colored("[!] Error: ", Color::Red);
                    println!("Ivalid theme name");
                    continue;
                }

                let mut path = std::env::var_os("HOME").expect("home env var is not set, HOME env variable is used to determine card vault");
                path.push("/.rcard_vault/");
                path.push(command[1]);
                path.push(".rcard");
                let mut theme_file = match fs::File::create_new(path) {
                    Ok(file) => file,
                    Err(_) => {
                        print_colored("[!] Error: ", Color::Red);
                        println!("Theme exists");
                        continue;
                    }
                };
                print_colored("w ", Color::Green);
                println!("to save and exit");
                let mut cards: Vec<Card> = Vec::new();
                let mut n = 0;
                loop {
                    n += 1;
                    print_colored("[#] ", Color::Blue);
                    println!("Front {n}: ");
                    let front = input_prompt();
                    println!();

                    if front.clone().trim() == "w" {
                        if cards.len() > 0 {
                            let theme: Theme = Theme { name: command[1].to_owned(), cards };
                            let card_data = serde_json::to_string(&theme).unwrap();
                            theme_file.write_all(card_data.as_bytes()).unwrap();
                        }
                        break;
                    }

                    print_colored("[#] ", Color::Blue);
                    println!("Reverse: ");
                    let back = input_prompt();
                    println!();

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
                }
            }
            "list" | "ls"  => {
                let themes = &get_themes();
                print_themes(themes);
            },
            "quit" | "exit" | "q" => exit_console(),
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
    ctrlc::set_handler(||{}).expect("error unsetting ctrl_c handler");
    execute!(stdout(), terminal::EnterAlternateScreen).unwrap();
    execute!(stdout(), terminal::Clear(ClearType::All)).unwrap();
    execute!(stdout(), SavePosition, MoveTo(0,0), EnableBlinking).unwrap();
    let themes = get_themes();  
    print_colored("Themes:\n", Color::Cyan);
    print_themes(&themes);
    main_console();
}
