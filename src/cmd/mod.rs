use crate::app::State;

pub mod help;
pub mod exit;
pub mod invalid;
pub mod null;

pub trait Command {
    fn get_name(&self) -> String;
    fn get_help(&self) -> String;
    fn handle(&self, state: &mut State, command: &str, args: &[&str]); 
}