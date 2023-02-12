use std::{error::Error};

use clap::Parser;

pub mod app;

fn main() -> Result<(), Box<dyn Error>> {
    app::run()?;
    Ok(())
}