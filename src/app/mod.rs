use std::{io::{Write, BufReader}, fs::File, path::{Path, PathBuf}, env::{self}, error::Error};
use serde::Deserialize;

use crate::cmd::{exit::{self, ExitCommand}, help::{self, HelpCommand}, Command};

pub fn run() -> Result<(), Box<dyn Error>> {
    let commands: Vec<Box<dyn Command>> = vec![Box::new(HelpCommand::new()), Box::new(ExitCommand::new())];
    let state = State::new("Provenance Shell", commands);

    let shell = env::var("PIO_SHELL")?;
    let path = Path::new(&shell).join("config.json");
    let config = get_config(path);

    let mut app = App::new(state, config);
    app.start();
    Ok(())
}

fn render_greeting() {
    println!("Welcome to Provenance Shell.");
    println!("Press tab or type help to see a list of commands.");
}

#[derive(Debug, Deserialize)]
pub struct Config {
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

pub struct App {
    state: State,
    _config: Config,
}

impl App {
    pub fn new(state: State, config: Config) -> Self {
        Self {
            state,
            _config: config,
        }
    }

    pub fn start(&mut self) {
        self.state.running = true;

        render_greeting();
        while self.state.running {
            render_input();
            let input = get_input();
            if input == "exit" {
                exit::handle(&mut self.state);
            } else if input == "help" {
                help::handle(&mut self.state);
            }
        }
    }

    pub fn stop(&mut self) {
        self.state.running = false;
    }

    pub fn get_name(&self) -> String {
        return self.state.get_name();
    }
}

pub struct State {
    pub running: bool,
    pub name: String,
    pub commands: Vec<Box<dyn Command>>
}

impl State {
    pub fn new(name: &str, commands: Vec<Box<dyn Command>>) -> Self {
        Self {
            running: false,
            name: name.to_string(),
            commands
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}