use std::{
    collections::BTreeMap,
    io::{Read, Seek},
};

use miniserde::Deserialize;
use serde::Deserialize as SerdeDeserialize;

#[derive(Debug, Deserialize, SerdeDeserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mrpack {
    pub format_version: u32,
    pub game: String,
    pub version_id: String,
    pub name: String,
    pub summary: Option<String>,
    pub files: Vec<File>,
    pub dependencies: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize, SerdeDeserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub path: String,
    pub hashes: BTreeMap<String, String>,
    pub env: Option<Environment>,
    pub downloads: Vec<String>,
    pub file_size: u32,
}

#[derive(Debug, Deserialize, SerdeDeserialize)]
pub struct Environment {
    pub client: EnvEnum,
    pub server: EnvEnum,
}

#[derive(Debug, Deserialize, SerdeDeserialize)]
#[serde(rename_all = "lowercase")]
pub enum EnvEnum {
    Required,
    Optional,
    Unsupported,
}

fn main() {
    let serde: Mrpack = dbg!(serde_json::from_str(include_str!("../modrinth.index.json")).unwrap());
    // Panics
    let miniserde: Mrpack =
        dbg!(miniserde::json::from_str(include_str!("../modrinth.index.json")).unwrap());
}
