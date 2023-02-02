
use crate::app::State;

use super::Command;



pub struct HelpCommand {

}

impl HelpCommand {
    pub fn new() -> Self {
        Self{}
    }
}

impl Command for HelpCommand {
    fn handle(&self, _state: &mut State, _command: &str, _args: &[&str]) {
        println!("help\texit");
    }

    fn get_name(&self) -> String {
        return "help".to_string();
    }

    fn get_help(&self) -> String {
        return "Displays this".to_string();
    }
}