use crate::app::{State, config::Config};

use super::Command;


#[derive(Clone)]
pub struct GetConfigCommand {
}

impl GetConfigCommand {
    pub fn new() -> Self {
        Self{
        }
    }
}

impl Command for GetConfigCommand {
    fn handle(&self, _state: &mut State, config: &mut Config, _command: &str, args: &[&str]) {
        if args.len() != 1 {
            println!("{} requires a [confiugration_key]", self.get_name());
            return
        }

        let key = args[0];

        match config.get(key) {
            None => println!("{} does not exist.", key),
            Some(val) => println!("{}", val)
        };
    }

    fn get_name(&self) -> String {
        return "get".to_string();
    }

    fn get_help(&self) -> String {
        return "This command allows you to get the value of the configuration key".to_string();
    }

    fn get_subcommands(&self) -> Vec<Box<dyn Command>> {
        vec![]
    }

    fn clone_box(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }
}