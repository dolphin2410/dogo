use std::error::Error;

use clap::{Subcommand, Parser};

use self::{github::handle_github_command, set::handle_set_command, search::handle_search_command, repo::handle_set_repo};

mod github;
mod repo;
mod search;
mod set;

#[derive(Subcommand, Debug)]
enum SubCommand {
    GITHUB {
        target: String
    },
    SET {
        variable: String,
        target: String
    },
    SEARCH {
        variable: String
    },
    REPO {
        selector: String,
        target: String
    }
}

#[derive(Parser, Debug)]
#[clap(author = "dolphin2410", version = "v1.0.0-beta", about = None, long_about = None)]
struct Args {
   #[clap(subcommand)]
   cmd: Option<SubCommand>
}

pub fn register_commands() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    if args.cmd.is_none() {
        println!("No Command");
        return Err("No Command".into());
    }

    match &args.cmd.unwrap() {
        SubCommand::GITHUB { target } => handle_github_command(target),
        SubCommand::SET { variable, target } => handle_set_command(variable, target),
        SubCommand::SEARCH { variable } => handle_search_command(variable)?,
        SubCommand::REPO { selector, target } => handle_set_repo(selector, target)?
    }

    Ok(())
}