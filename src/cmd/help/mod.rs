
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
    fn handle(&self, state: &mut State) {
        println!("Exiting {}", state.get_name());
        state.running = false;
    }

    fn get_name(&self) -> String {
        return "help".to_string();
    }

    fn get_help(&self) -> String {
        return "Displays this".to_string();
    }
}