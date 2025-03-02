use std::{fs::File, io::{BufReader, Write}, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Card {
    pub front: String,
    pub back: String
}

#[derive(Serialize, Deserialize)]
pub struct Deck {
    pub name: String,
    pub cards: Vec<Card>
}

pub enum DeckError {
    Io(std::io::Error),
    Serde(serde_json::Error)
}

impl Deck {
    pub fn new(name: String, cards: Vec<Card>) -> Self {
        Self { name, cards }
    }

    pub fn from_path(path: impl AsRef<Path>) -> Result<Deck, DeckError> {
        let file = File::open(&path).map_err(DeckError::Io)?;
        let reader = BufReader::new(file);
        let deck: Deck = serde_json::from_reader(reader).map_err(DeckError::Serde)?;

        Ok(deck)
    }

    pub fn save(self, path: impl AsRef<Path>) -> Result<(), DeckError> {
        let mut deck_file = File::create_new(&path).map_err(DeckError::Io)?;
        let card_data = serde_json::to_string(&self).unwrap();
        deck_file.write_all(card_data.as_bytes()).unwrap();

        Ok(())
    }
}
