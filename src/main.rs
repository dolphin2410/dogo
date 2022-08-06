use clap::{Parser,Subcommand};
use colored::Colorize;
use fancy_regex::Regex;
use serde::{Serialize, Deserialize};
use url::Url;

#[derive(Deserialize, Serialize)]
struct GitRepo {
    default_branch: String
}

#[derive(Subcommand, Debug)] 
enum SubCommand {
    GRADLE {
        target: String
    }
}

#[derive(Parser, Debug)]
#[clap(author = "dolphin2410", version = "v1.0.0-beta", about = None, long_about = None)]
struct Args {
   #[clap(subcommand)]
   cmd: Option<SubCommand>
}

fn main() {
   let args = Args::parse();
    match &args.cmd {
        Some(SubCommand::GRADLE { target }) => handle_java(target),
        None => println!("No Command")
    }

}

fn handle_java(target: &String) {
    if let Ok(url) = Url::parse(target) {
        get_from_url(url);
    } else {
        let is_gh = Regex::new(r"@\w+/\w+").unwrap().is_match(target).unwrap();
        if is_gh {
            let repo_detail = target.strip_prefix("@").unwrap();
            let repo = get_repo(Url::parse(format!("https://api.github.com/repos/{}", repo_detail).as_str()).unwrap());
            let url_str = format!("https://raw.githubusercontent.com/{}/{}/README.md", repo_detail, repo.default_branch);

            if let Ok(url) = Url::parse(&url_str) {
                println!("\n\n");
                println!("{}: https://github.com/{}/blob/{}/README.md", format!("README").blue(), repo_detail, repo.default_branch);
                get_from_url(url);
            } else {
                println!("Invalid URL");
            }
        } else {
            println!("Who are you?");
        }
    }
}

fn get_repo(git_repo: Url) -> GitRepo {
    let body = ureq::get(git_repo.as_str()).call().unwrap().into_string().unwrap();
    serde_json::from_str::<GitRepo>(&body).unwrap()
}

fn get_from_url(url: Url) {
    let mut body = ureq::get(url.as_str()).call().unwrap().into_string().unwrap();
    body.retain(|c| !c.is_whitespace() && c != '\n' && c != '\r' && c != '\t');
    
    let repo = parse_repo(&body).unwrap();
    let artifact = parse_artifact(&body).unwrap();
    println!("{}: {}", format!("Repository").blue(), repo);
    println!("{}: {}", format!("Artifact").blue(), artifact);
    println!("\n\n");
}

fn fetch_latest_version(group: &str, artifact: &str) -> String {
    let body = ureq::get(format!("https://repo.maven.apache.org/maven2/{}/{}/maven-metadata.xml", group.replace(".", "/"), artifact).as_str()).call().unwrap().into_string().unwrap();
    let matches = Regex::new(r#"(?<=<latest>)(.*?)(?=<\/latest>)"#).unwrap().find(&body).unwrap();
    String::from(matches.unwrap().as_str())
}

fn parse_repo(str: &String) -> Result<&str, &str> {
    let regex = Regex::new(r#"(?<=repositories{maven{url=uri\(").*(?="\)}})"#).unwrap().find(str).unwrap();
    let regex_groovy = Regex::new(r#"(?<=repositories{maven{url").*(?="}})"#).unwrap().find(str).unwrap();
    let regex_short = Regex::new(r#"(?<=repositories{maven\(").*(?="\)})"#).unwrap().find(str).unwrap();
    let regex_central = Regex::new(r#"repositories{mavenCentral\(\)}"#).unwrap().is_match(str).unwrap();
    if regex_central {
        return Ok("mavenCentral");
    }
    if let Some(repository) = regex {
        Ok(repository.as_str())
    } else {
        if let Some(repository) = regex_groovy {
            Ok(repository.as_str())
        } else {
            if let Some(repository) = regex_short {
                Ok(repository.as_str())
            } else {
                Err("Could not find any repo")
            }
        }
    }
}

fn parse_artifact(str: &String) -> Result<String, &str> {
    let regex_impl_p = Regex::new(r#"(?<=implementation\(").*(?="\))"#).unwrap().find(str).unwrap();
    let regex_compile_p = Regex::new(r#"(?<=compileOnly\(").*(?="\))"#).unwrap().find(str).unwrap();
    let regex_impl = Regex::new(r#"(?<=implementation").*(?=")"#).unwrap().find(str).unwrap();
    let regex_compile = Regex::new(r#"(?<=compileOnly").*(?=")"#).unwrap().find(str).unwrap();
    let repo = if let Some(repository) = regex_impl {
        repository.as_str()
    } else {
        if let Some(repository) = regex_impl_p {
            repository.as_str()
        } else {
            if let Some(repository) = regex_compile {
                repository.as_str()
            } else {
                if let Some(repository) = regex_compile_p {
                    repository.as_str()
                } else {
                    ""
                }
            }
        }
    };
    if repo != "" {
        if repo.contains("<version>") {
            let mut split = repo.split(":");
            let version = fetch_latest_version(split.nth(0).unwrap(), split.nth(0).unwrap());
            let replace = repo.replace("<version>", version.clone().as_str());
            Ok(replace)
        } else {
            Ok(String::from(repo))
        }
    } else {
        Err("Could not find any artifact")
    }
}