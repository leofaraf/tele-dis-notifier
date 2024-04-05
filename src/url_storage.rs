use std::{fs::File, io::{BufReader, Write}};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Storage {
    default_url: String,
    urls: HashMap<String, String>
}

impl Default for Storage {
    fn default() -> Self {
        Self { 
            default_url: Default::default(),
            urls: Default::default()
        }
    }
}

pub struct UrlStorage;
const URL_STORAGE_PATH: &str = "url_storage.json";

impl UrlStorage {
    fn get_map() -> Storage {
        let file = match File::open(URL_STORAGE_PATH) {
            Ok(file) => file,
            Err(_) => {
                Self::save(Storage::default());
                return Self::get_map();
            },
        };
        let reader = BufReader::new(file);

        let map: Storage = serde_json::from_reader(reader).unwrap();
        map
    }

    fn save(storage: Storage) -> File {
        let mut file = File::create(URL_STORAGE_PATH).expect("Can't create a file!");
        file.write(&serde_json::to_string(&storage).unwrap().as_bytes()).expect("Can't write to a file!");
        file
    }

    pub fn set_url(id: String, url: String) {
        let mut storage = Self::get_map();
        storage.urls.insert(id, url);
        
        Self::save(
            storage
        );
    }

    pub fn set_default_url(url: String) {
        let mut storage = Self::get_map();
        storage.default_url = url;
        
        Self::save(
            storage
        );
    }

    pub fn get_url(id: String) -> String {
        let map = Self::get_map();
        map.urls
        .iter()
        .find(|e| e.0 == &id)
        .and_then(|e| Some(e.1.clone()))
        .unwrap_or(map.default_url)
    }
}