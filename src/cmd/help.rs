
use crate::app::{State, config::Config};

use super::Command;


#[derive(Clone)]
pub struct HelpCommand {

}

impl HelpCommand {
    pub fn new() -> Self {
        Self{}
    }
}

impl Command for HelpCommand {
    fn handle(&self, _state: &mut State, _config: &mut Config, _command: &str, _args: &[&str]) {
        println!("help\texit");
    }

    fn get_name(&self) -> String {
        return "help".to_string();
    }

    fn get_help(&self) -> String {
        return "Displays this".to_string();
    }

    fn get_subcommands(&self) -> Vec<Box<dyn Command>> {
        vec![]
    }

    fn clone_box(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }
}