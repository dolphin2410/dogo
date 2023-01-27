use std::error::Error;

use crate::{database::{get_notation, get_repo}, maven::search_maven};

pub fn handle_search_command(selector: &str) -> Result<(), Box<dyn Error>> {
    let target = get_notation(selector).ok_or("Invalid Variable")?;
    let repo = get_repo(&selector);
    let meta = search_maven(&target, &repo)?;

    println!("{}:{}:{}", meta.group_id, meta.artifact_id, meta.versioning.latest);
    Ok(())
}