use crate::app::State;

use super::Command;



pub struct ExitCommand {

}

impl ExitCommand {
    pub fn new() -> Self {
        Self{}
    }
}

impl Command for ExitCommand {
    fn handle(&self, state: &mut State) {
        println!("Exiting {}", state.get_name());
        state.running = false;
    }

    fn get_name(&self) -> String {
        return "exit".to_string();
    }

    fn get_help(&self) -> String {
        return "Exits the shell".to_string();
    }
}