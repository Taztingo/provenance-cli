use crate::app::State;
use crate::app::config::Config;

pub mod help;
pub mod exit;
pub mod invalid;
pub mod null;
pub mod config;

pub trait Command {
    fn get_name(&self) -> String;
    fn get_help(&self) -> String;
    fn handle(&self, state: &mut State, config: &mut Config, command: &str, args: &[&str]);
    fn get_subcommands(&self) -> Vec<Box<dyn Command>>;
    fn clone_box(&self) -> Box<dyn Command>;
}

impl Clone for Box<dyn Command> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}