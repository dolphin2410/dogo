use std::{error::Error, fs::File, io::{BufWriter, Write, BufReader}};

use serde::{Serialize, Deserialize};
use url::Url;

#[derive(Serialize, Deserialize)]
pub struct VariableData {
    pub name: String,
    pub repository: String,
    pub notation: String
}

pub fn update_variables(data: Vec<VariableData>) {
    let path = std::env::current_exe().unwrap().parent().unwrap().join("db.json");
    let db = File::create(path).unwrap();
    let mut buf = BufWriter::new(db);
    serde_json::to_writer_pretty(&mut buf, &data).unwrap();
}

pub fn list_variables() -> Vec<VariableData> {
    let path = std::env::current_exe().unwrap().parent().unwrap().join("db.json");
    if !path.clone().exists() {
        let write = File::create(path.clone()).unwrap();
        let mut buf = BufWriter::new(write);
        buf.write_all("[]".as_bytes()).unwrap();
    }
    let read = File::open(path).unwrap();
    
    let res = serde_json::from_reader(BufReader::new(read));
    res.unwrap()
}

pub fn create_variable(name: &str, repo: &str, notation: &str) {
    let mut data = list_variables();
    data.push(VariableData {
        name: name.to_string(),
        repository: repo.to_string(),
        notation: notation.to_string()
    });
    update_variables(data);
}

pub fn get_notation(selector: &str) -> Option<String> {
    let mut result = None;

    for data in list_variables().iter() {
        if data.name == selector {
            result = Some(data.notation.clone());
        }
    }

    result
}

/// variable: nickname e.g) @dolphin2410/mcphysics
/// target: target notation e.g) io.github.dolphin2410:mcphysics 
pub fn set_notation(variable: &str, target: &str) {
    let mut data = list_variables();

    for datum in data.iter_mut() {
        if datum.name == variable {
            datum.notation = target.to_string();
        }
    }

    update_variables(data);
}

pub fn set_repo(variable: &str, target: &str) {
    let mut data = list_variables();

    for datum in data.iter_mut() {
        if datum.name == variable {
            datum.repository = target.to_string();
        }
    }

    update_variables(data);
}

pub fn get_repo(variable: &str) -> String {
    let mut result = None;

    for data in list_variables().iter() {
        if data.name == variable {
            result = Some(data.repository.clone());
        }
    }

    validate_repo(&result.unwrap_or("mavenCentral".to_string())).unwrap()
}

pub fn validate_repo(repo: &str) -> Result<String, Box<dyn Error>> {
    if repo.to_lowercase() == "mavencentral" {
        Ok("https://repo.maven.apache.org/maven2".to_string())
    } else if repo.to_lowercase() == "mavenlocal" {
        todo!()
    } else if is_valid_url(repo) {
        Ok(repo.to_string())
    } else {
        Err("Invalid Repository URL".into())
    }
}

/// check if it is a valid url
fn is_valid_url(repo: &str) -> bool {
    let url = Url::parse(repo);
    if url.is_err() {
        return false;
    }
    let url = url.unwrap();
    if url.scheme() != "http" && url.scheme() != "https" {
        return false;
    }
    true
}