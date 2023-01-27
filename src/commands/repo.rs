use std::error::Error;

use crate::database::{list_variables, set_repo, validate_repo, get_notation};

pub fn handle_set_repo(selector: &str, target: &str) -> Result<(), Box<dyn Error>> {
    let repo = validate_repo(target)?;
    
    if selector == "@a" {
        // select all variables
        let variables = list_variables();
        for var in variables {
            set_repo(&var.name, &repo);
        }
    } else if get_notation(selector).is_some() {
        // select a single variable
        set_repo(&selector, &repo);
    } else {
        // variable doesn't exist
        println!("Invalid variable")
    }

    Ok(())
}