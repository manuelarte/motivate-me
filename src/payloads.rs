use std::time::SystemTime;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")] // Converts enum variants to lowercase
pub enum StarAction {
    Created, Deleted
}


#[derive(Debug, Deserialize)]
pub struct StarPayload {
    action: String,
    repository: String,
    sender: String,
    starred_at: SystemTime
}

#[derive(Debug, Deserialize)]
pub struct Forkee {
    allow_forking : bool,
    archived: bool,
    branches_url: String,
    clone_url: String,
    created_at: SystemTime,
    fork: bool,
    forks: i8,
    forks_count: i8,
    name: String,
    stargazers: i8,
    stargazers_count: i8,

}

#[derive(Debug, Deserialize)]
pub struct ForkPayload {
    forkee: Forkee,
    repository: String,
    sender: String
}
