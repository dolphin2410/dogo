use std::error::Error;

use dogo::{commands::register_commands};

fn main() -> Result<(), Box<dyn Error>> {
    register_commands()?;
    Ok(())
}