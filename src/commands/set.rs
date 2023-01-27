use crate::database::{create_variable};

pub fn handle_set_command(variable: &str, target: &str) {
    create_variable(variable, "mavenCentral", target)
}