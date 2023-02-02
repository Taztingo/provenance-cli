use crate::app::{State, config::Config};

use super::Command;


#[derive(Clone)]
pub struct InvalidCommand {

}

impl InvalidCommand {
    pub fn new() -> Self {
        Self{}
    }
}

impl Command for InvalidCommand {
    fn get_name(&self) -> String {
        return "invalid".to_string();
    }

    fn get_help(&self) -> String {
        return "An invalid command.".to_string();
    }

    fn handle(&self, _state: &mut State, _config: &mut Config, command: &str, _args: &[&str]) {
        println!("'{}' command is not recognized", command);
    }

    fn get_subcommands(&self) -> Vec<Box<dyn Command>> {
        vec![]
    }

    fn clone_box(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }
}