use std::error::Error;

pub mod app;
pub mod cmd;


fn main() -> Result<(), Box<dyn Error>> {
    app::run()?;
    Ok(())
}