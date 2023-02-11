use std::{io::{Write}, path::{Path, PathBuf}, env::{self}, error::Error};

mod config;

pub fn run() -> Result<(), Box<dyn Error>> {

    let shell = env::var("PIO_SCRIPT")?;
    let path = Path::new(&shell).join("config.json");
    let config = config::get_config(&path);
    let state = State::new("Provenance Script",  path);

    let mut app = App::new(state, config);
    Ok(())
}

pub struct App {
    state: State,
    config: config::Config,
}

impl App {
    pub fn new(state: State, config: config::Config) -> Self {
        Self {
            state,
            config: config,
        }
    }

    pub fn get_name(&self) -> String {
        return self.state.get_name();
    }
}

pub struct State {
    pub running: bool,
    pub name: String,
    pub config_path: PathBuf
}

impl State {
    pub fn new(name: &str, config_path: PathBuf) -> Self {
        Self {
            running: false,
            name: name.to_string(),
            config_path,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}