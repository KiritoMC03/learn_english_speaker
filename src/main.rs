#![windows_subsystem = "windows"]
use std::{path::Path, fs::File, io::{BufReader, BufRead}, time::Duration};

use tts_rust::{ tts::GTTSClient, languages::Languages };

const WORDS_DELAY : u64 = 1;
const ITEMS_DELAY : u64 = 3;
const REPEATS_DELAY : u64 = 600;
const INITIAL_DELAY : u64 = 300;

struct Translate {
    rus: String,
    eng: String,
}

fn main() {
    let rus_speaker: GTTSClient = GTTSClient {
        volume: 1.0,
        language: Languages::Russian,
        tld: "com",
    };

    let eng_speaker: GTTSClient = GTTSClient {
        volume: 1.0,
        language: Languages::English,
        tld: "com",
    };

    std::thread::sleep(Duration::from_secs(INITIAL_DELAY));
    loop {
        let tanslates = load_translates();
        for item in &tanslates {
            println!("{} - {}", item.rus, item.eng);
            say_trananslate(item, &rus_speaker, &eng_speaker);
            std::thread::sleep(Duration::from_secs(ITEMS_DELAY));
        }
        drop(tanslates);
        std::thread::sleep(Duration::from_secs(REPEATS_DELAY));
    }
}

fn load_translates() -> Vec<Translate> {
    let mut result = Vec::new();
    let file = open_translates(translates_path());
    for (i, line) in BufReader::new(file).lines().enumerate() {
        match line {
            Ok(str) => {
                if let Some(trsl) = Translate::from_split(str.split("-")) {
                    result.push(trsl);
                }
            },
            Err(e) => eprintln!("Can`t read line number {} with error {e}", i + 1),
        }
    }

    result
}

fn translates_path() -> String {
    format!("{}\\translates.txt", std::env::current_dir().unwrap().display())
}

fn open_translates(path: String) -> File {
    if !Path::new(path.as_str()).exists(){
        match File::create(path.clone()) {
            Ok(f) => { return f },
            Err(e) => eprintln!("Can`t create default translates file with error: {}", e),
        };
    }

    File::open(path).unwrap()
}

fn say_trananslate(target: &Translate, rus_sp: &GTTSClient, eng_sp: &GTTSClient) {
    match rus_sp.speak(target.rus.as_str()) {
        Ok(_) => {
            std::thread::sleep(Duration::from_secs(WORDS_DELAY));
            eng_sp.speak(target.eng.as_str());
        },
        Err(_) => { },
    }
}

impl Translate {
    pub fn from_split(mut split: std::str::Split<&str>) -> Option<Translate> {
        if let Some(rus) = split.next() {
            if let Some(eng) = split.next() {
                return Some(Translate {
                    rus: rus.to_string(),
                    eng: eng.to_string(),
                });
            }
        }

        None
    }
}