use std::{path::PathBuf, fs::File, io::{BufReader, Write}, collections::HashMap, error::Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    provenance_binary: String,
    external_scripts: String,
    provenance_home: String,
    test_network: String,
    gas_prices: String,
    gas_adjustment: String
}

pub fn get_config(path: &PathBuf) -> Config {
    let file = File::open(&path).expect(format!("{} does not exist!", &path.display()).as_str());
    let reader = BufReader::new(file);
    let config: Config = serde_json::from_reader(reader).expect(format!("{} is not valid json format!", path.display()).as_str());
    config
}

impl Config {
    pub fn set(&mut self, key: &str, value: &str) -> bool {
        let mut map = self.to_hashmap();
        if !map.contains_key(key) {
            println!("{:?} does not contain {}", map, key);
            return false;
        }
        map.insert(key.to_string(), value.to_string());

        let config = self.to_struct(&map);
        let _ = std::mem::replace(self, config);
        return true;
    }

    pub fn save(&self, path: &PathBuf) {
        let json = serde_json::to_string(&self).unwrap();
        let mut file = File::create(path).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let map = self.to_hashmap();
        match map.get(key) {
            None => None,
            Some(value) => Some(value.clone())
        }
    }

    pub fn get_keys(&self) -> Vec<String> {
        self.to_hashmap().into_keys().collect()
    }

    fn to_hashmap(&self) -> HashMap<String, String> {
        let serialized = serde_json::to_string(&self).unwrap();
        serde_json::from_str(&serialized).unwrap()
    }

    fn to_struct(&mut self, map: &HashMap<String, String>) -> Config {
        let serialized = serde_json::to_string(&map).unwrap();
        serde_json::from_str(&serialized).unwrap()
    }
    
}