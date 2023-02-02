use crate::app::{State, config::Config};

use super::Command;


#[derive(Clone)]
pub struct ExitCommand {

}

impl ExitCommand {
    pub fn new() -> Self {
        Self{}
    }
}

impl Command for ExitCommand {
    fn handle(&self, state: &mut State, _config: &mut Config, _command: &str, _args: &[&str]) {
        println!("Exiting {}", state.get_name());
        state.running = false;
    }

    fn get_name(&self) -> String {
        return "exit".to_string();
    }

    fn get_help(&self) -> String {
        return "Exits the shell.".to_string();
    }

    fn get_subcommands(&self) -> Vec<Box<dyn Command>> {
        vec![]
    }

    fn clone_box(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }
}