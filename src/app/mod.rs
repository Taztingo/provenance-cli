use std::{io::{Write}, path::{Path, PathBuf}, env::{self}, error::Error};

mod config;
mod cli;

pub fn run() -> Result<(), Box<dyn Error>> {

    let shell = env::var("PIO_SCRIPT")?;
    let path = Path::new(&shell).join("config.json");
    let mut config = config::get_config(&path);
    let state = State::new("Provenance Script",  path);

    let mut app = App::new(state, config);
    cli::cli(&mut app);
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

    pub fn get_config(&mut self) -> &mut config::Config {
        &mut self.config
    }

    pub fn get_config_path(&self) -> &PathBuf {
        &self.state.config_path
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