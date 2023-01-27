use std::error::Error;

use serde::{Serialize, Deserialize};

pub fn search_maven(notation: &str, repo: &str) -> Result<Metadata, Box<dyn Error>> {
    let notation_path = notation.replace(':', "/").replace('.', "/");

    let conjunction = if !repo.ends_with("/") {
        "/"
    } else {
        ""
    };

    let url = format!("{}{}{}/maven-metadata.xml", repo, conjunction, notation_path);

    let resp = reqwest::blocking::get(url)?;

    let reader = &resp.bytes()?[..];

    let mut de = serde_xml_rs::Deserializer::new_from_reader(reader).non_contiguous_seq_elements(true);

    let xml = Metadata::deserialize(&mut de)?;

    Ok(xml)
}

#[derive(Serialize, Deserialize)]
pub struct Versions {
    pub version: Vec<String>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Versioning {
    pub latest: String,
    pub versions: Versions,
    pub last_updated: u64
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub group_id: String,
    pub artifact_id: String,
    pub versioning: Versioning
}