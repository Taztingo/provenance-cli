use crate::app::{State, config::Config};

use super::Command;


#[derive(Clone)]
pub struct SetConfigCommand {
}

impl SetConfigCommand {
    pub fn new() -> Self {
        Self{
        }
    }
}

impl Command for SetConfigCommand {
    fn handle(&self, state: &mut State, config: &mut Config, _command: &str, args: &[&str]) {
        if args.len() != 2 {
            println!("{} requires a [confiugration_key] and a [configuration_value]", self.get_name());
            return
        }

        let key = args[0];
        let value = args[1];

        if !config.set(key, value) {
            println!("{} is not a valid configuration key", &key);
            return
        }
        config.save(&state.config_path);
    }

    fn get_name(&self) -> String {
        return "set".to_string();
    }

    fn get_help(&self) -> String {
        return "This command allows you to set the value of the configuration key".to_string();
    }

    fn get_subcommands(&self) -> Vec<Box<dyn Command>> {
        vec![]
    }

    fn clone_box(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }
}