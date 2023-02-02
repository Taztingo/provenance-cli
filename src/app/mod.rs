use std::{io::{Write}, path::{Path}, env::{self}, error::Error};

use crate::cmd::{exit::{ExitCommand}, help::{HelpCommand}, Command, invalid::InvalidCommand, null::NullCommand};
pub mod config;

pub fn run() -> Result<(), Box<dyn Error>> {
    let commands: Vec<Box<dyn Command>> = vec![Box::new(HelpCommand::new()), Box::new(ExitCommand::new())];
    let state = State::new("Provenance Shell", commands);

    let shell = env::var("PIO_SHELL")?;
    let path = Path::new(&shell).join("config.json");
    let config = config::get_config(path);

    let mut app = App::new(state, config);
    app.start();
    Ok(())
}

fn render_greeting() {
    println!("Welcome to Provenance Shell.");
    println!("Press tab or type help to see a list of commands.");
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
    config: config::Config,
}

impl App {
    pub fn new(state: State, config: config::Config) -> Self {
        Self {
            state,
            config: config,
        }
    }

    pub fn start(&mut self) {
        self.state.running = true;

        render_greeting();
        while self.state.running {
            render_input();
            let mut cmd: Box<dyn Command> = Box::new(InvalidCommand::new());
            let raw_input = get_input();
            let mut args: Vec<&str> = raw_input.split_whitespace().collect();
            let mut input = "";
            if args.len() == 0 {
                cmd = Box::new(NullCommand::new());
            } else {
                input = args.remove(0);
                if input == "exit" {
                    cmd = Box::new(ExitCommand::new());
                } else if input == "help" {
                    cmd = Box::new(HelpCommand::new());
                }
            }

            cmd.handle(&mut self.state, &mut self.config, input, &args);
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