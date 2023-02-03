use crate::app::{State, config::Config};

use super::Command;

pub mod set_config;
pub mod get_config;

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
    fn handle(&self, state: &mut State, config: &mut Config, _command: &str, args: &[&str]) {
        // Have it require a subcommand
        if args.len() == 0 {
            println!("'{}' takes a required subcommand. Must either `get` or `set`.", self.get_name());
            return
        }

        let mut subcommand_args: Vec<&str> = args.to_vec();
        let subcommand_name = subcommand_args.remove(0);
        for subcommand in &self.subcommands {
            if subcommand.get_name() == subcommand_name {
                subcommand.handle(state, config, subcommand_name, &subcommand_args);
            }
        }
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