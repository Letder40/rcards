use std::{fs::{self, DirEntry, ReadDir}, io::{self, Error, Result}, path::{Path, PathBuf}, time::SystemTime};

use super::print::{print_flag, Flags};

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

pub fn get_decks() -> Vec<String> {
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

pub fn deck_path_from_id(id: usize) -> Result<PathBuf> {
    let mut path = std::env::var_os("HOME").expect("home env var is not set, HOME env variable is used to determine card vault");
    let decks = get_decks();
    if decks.len() - 1 >= id {
        path.push("/.rcard_vault/");
        path.push(&decks[id]);
        path.push(".rcard");
    } else {
        return Err(Error::new(io::ErrorKind::NotFound, "Id not found"));
    }

    let path = PathBuf::from(path);

    if !path.exists() {
        return Err(Error::new(io::ErrorKind::NotFound, "File not found"));
    }

    Ok(path)
}

pub fn deck_path_from_name(name: &str) -> Result<PathBuf> {
    let mut path = std::env::var_os("HOME").expect("home env var is not set, HOME env variable is used to determine card vault");
    path.push("/.rcard_vault/");
    path.push(name);
    path.push(".rcard");

    let path = PathBuf::from(path);

    if !path.exists() {
        return Err(Error::new(io::ErrorKind::NotFound, "Id not found"));
    }

    Ok(path)
}

pub fn get_deckpath(guess: &str) -> Result<PathBuf> {
    let path = if guess.parse::<usize>().is_ok() {
        let id: usize = guess.parse().unwrap();
        deck_path_from_id(id)
    } else {
        deck_path_from_name(guess)
    };

    if path.is_err() {
        print_flag(Flags::Error, "deck not exists");
    }

    return path;
}
