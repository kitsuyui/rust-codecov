use crate::author::Author;
use crate::url::Url;
/**
 * Codecov v2 API
 * /repos endpoint returns a list of repos for a given owner.
 */
use serde::{Deserialize, Serialize};

/**
 * ReposAPIResponse is a struct that represents the response from the repos API.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct ReposAPIResponse {
    pub results: Vec<Repo>,
    pub count: usize,
    pub next: Option<Url>,
    pub previous: Option<Url>,
    pub total_pages: usize,
}

/**
 * Repo is a struct that represents a repo.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct Repo {
    pub name: String,
    pub private: bool,
    pub updatestamp: Option<String>, // TODO: ISO Date
    pub author: Author,
    pub language: Option<String>,
    pub branch: Option<String>,
    pub active: bool,
    pub activated: bool,
}
