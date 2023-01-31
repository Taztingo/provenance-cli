use std::{io::{Write, BufReader}, fs::File, path::{Path, PathBuf}, env};
use serde::Deserialize;

pub fn run() {
    let name = "Provenance Shell";
    let shell = env::var("PIO_SHELL").unwrap();
    let path = Path::new(&shell).join("config.json");
    let config = get_config(path);
    render_greeting();
    loop {
        render_input();
        let input = get_input();
        if input == "exit" {
            println!("Exiting {}", name);
            break;
        }
    }
}

fn render_greeting() {
    println!("Welcome to Provenance Shell.");
    println!("Press tab or type help to see a list of commands.");
}

#[derive(Debug, Deserialize)]
struct Config {
    provenance_path: String,
    external_scripts_path: String,
}

fn get_config(path: PathBuf) -> Config {
    let file = File::open(&path).expect(format!("{} does not exist!", &path.display()).as_str());
    let reader = BufReader::new(file);
    let config: Config = serde_json::from_reader(reader).expect(format!("{} is not valid json format!", path.display()).as_str());
    config
}

fn render_input() {
    
    let user = whoami::username();
    let path = std::env::current_dir().unwrap();
    print!("{}:{}$ ", user, path.to_str().unwrap());
    std::io::stdout().flush().unwrap();
    
}

fn get_input() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}