use crate::app::{State, config::Config};

use super::Command;


#[derive(Clone)]
pub struct ConfigCommand {
    subcommands: Vec<Box<dyn Command>>
}

impl ConfigCommand {
    pub fn new(subcommands: Vec<Box<dyn Command>>) -> Self {
        Self{
            subcommands
        }
    }
}

impl Command for ConfigCommand {
    fn handle(&self, _state: &mut State, _config: &mut Config, _command: &str, _args: &[&str]) {
    }

    fn get_name(&self) -> String {
        return "config".to_string();
    }

    fn get_help(&self) -> String {
        return "This command allows you to get or set the config.".to_string();
    }

    fn get_subcommands(&self) -> Vec<Box<dyn Command>> {
        self.subcommands.clone()
    }

    fn clone_box(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }
}