use crate::app::State;

use super::Command;



pub struct NullCommand {

}

impl NullCommand {
    pub fn new() -> Self {
        Self{}
    }
}

impl Command for NullCommand {
    fn handle(&self, _state: &mut State, _command: &str, _args: &[&str]) {
    }

    fn get_name(&self) -> String {
        return "null".to_string();
    }

    fn get_help(&self) -> String {
        return "This command does nothing.".to_string();
    }
}