use colored::Colorize;
use fancy_regex::Regex;
use reqwest::{Url, blocking::Client};
use serde::{Serialize, Deserialize};

use crate::maven::{search_maven, Metadata};


#[derive(Deserialize, Serialize)]
struct GitRepo {
    default_branch: String
}


pub fn handle_github_command(target: &str) {
    if let Ok(url) = Url::parse(target) {
        get_from_url(url);
    } else {
        let is_gh = Regex::new(r"@\w+/\w+").unwrap().is_match(target).unwrap();
        if is_gh {
            let repo_detail = target.strip_prefix("@").unwrap();
            let repo = get_gh_repo(Url::parse(format!("https://api.github.com/repos/{}", repo_detail).as_str()).unwrap());
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

fn get_gh_repo(git_repo: Url) -> GitRepo {
    let client = Client::builder()
    .user_agent("gomgom")
    .build()
    .unwrap();
    let body = client.get(git_repo.as_str()).send().unwrap().text().unwrap();
    serde_json::from_str::<GitRepo>(&body).unwrap()
}

fn get_from_url(url: Url) {
    let mut body = reqwest::blocking::get(url.as_str()).unwrap().text().unwrap();
    body.retain(|c| !c.is_whitespace() && c != '\n' && c != '\r' && c != '\t');
    
    let repo = parse_repo(&body).unwrap();
    let artifact = parse_artifact(&body).unwrap();
    println!("{}: {}", format!("Repository").blue(), repo);
    println!("{}: {}", format!("Artifact").blue(), artifact);
    println!("\n\n");
}

fn fetch_latest_version(artifact: &str) -> Metadata {
    let metadata = search_maven(artifact, "https://repo.maven.apache.org/maven2/").unwrap();
    return metadata
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
            let split = repo.split(":").collect::<Vec<&str>>();
            let repo = format!("{}:{}", split[0], split[1]);
            let version = fetch_latest_version(&repo);
            Ok(format!("{}:{}:{}", version.group_id, version.artifact_id, version.versioning.latest))
        } else {
            Ok(String::from(repo))
        }
    } else {
        Err("Could not find any artifact")
    }
}