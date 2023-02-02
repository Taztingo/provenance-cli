use std::{path::PathBuf, fs::File, io::BufReader};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    provenance_binary: String,
    external_scripts: String,
    provenance_home: String,
    test_network: bool,
    gas_prices: u64,
    gas_adjustment: f64
}

pub(crate) fn get_config(path: PathBuf) -> Config {
    let file = File::open(&path).expect(format!("{} does not exist!", &path.display()).as_str());
    let reader = BufReader::new(file);
    let config: Config = serde_json::from_reader(reader).expect(format!("{} is not valid json format!", path.display()).as_str());
    config
}