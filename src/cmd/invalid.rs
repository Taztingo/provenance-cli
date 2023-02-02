use crate::app::State;

use super::Command;



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

    fn handle(&self, _state: &mut State, command: &str, _args: &[&str]) {
        println!("'{}' command is not recognized", command);
    }
}