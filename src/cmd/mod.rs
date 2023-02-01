use crate::app::State;

pub mod help;
pub mod exit;

pub trait Command {
    fn get_name(&self) -> String;
    fn get_help(&self) -> String;
    fn handle(&self, state: &mut State); 
}