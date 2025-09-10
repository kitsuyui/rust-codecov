/**
 * Codecov v2 API
 * /repos endpoint returns a list of repos for a given owner.
 */
use serde::{Deserialize, Serialize};

use crate::url::Url;

/**
 * CommitsAPIResponse is a struct that represents the response from the commits API.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct CommitsAPIResponse {
    pub results: Vec<Commit>,
    pub count: usize,
    pub next: Option<Url>,
    pub previous: Option<Url>,
    pub total_pages: usize,
}

/**
 * CommitAuthor is a struct that represents the author of a commit.
 * Note: This is different from the Author struct in src/author.rs.
 * name is optional in this struct.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct CommitAuthor {
    pub service: String,
    pub username: String,
    pub name: Option<String>,
}

/**
 * Commit is a struct that represents a commit.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct Commit {
    pub commitid: String,
    pub message: Option<String>,
    pub timestamp: Option<String>, // TODO: ISO Date
    pub ci_passed: bool,
    pub author: Option<CommitAuthor>,
    pub branch: Option<String>,
    pub totals: Totals,
    pub state: Option<String>,
    pub parent: Option<String>,
}

/**
 * Totals is a struct that represents the totals for a commit.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct Totals {
    pub files: Option<usize>,
    pub lines: Option<usize>,
    pub hits: Option<usize>,
    pub misses: Option<usize>,
    pub partials: Option<usize>,
    #[serde(default)]
    pub coverage: Option<f64>,
    pub branches: Option<usize>,
    pub methods: Option<usize>,
    pub sessions: Option<usize>,
    pub complexity: Option<f64>,
    pub complexity_total: Option<f64>,
    #[serde(default)]
    pub complexity_ratio: Option<f64>,
    pub diff: Option<usize>,
}

impl CommitsAPIResponse {
    pub fn coverage(&self) -> Option<f64> {
        if self.count == 0 {
            return None;
        }
        let mut total_coverage = 0.0;
        for commit in &self.results {
            total_coverage += commit.totals.coverage.unwrap_or(0.0);
        }
        Some(total_coverage / self.count as f64)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_coverage() {
        use super::*;
        let mut response = CommitsAPIResponse {
            results: vec![],
            count: 1,
            next: None,
            previous: None,
            total_pages: 1,
        };
        assert_eq!(response.coverage(), Some(0.0));
        let commit = Commit {
            commitid: String::from("123"),
            message: Some(String::from("message")),
            timestamp: Some(String::from("timestamp")),
            ci_passed: true,
            author: Some(CommitAuthor {
                service: String::from("service"),
                username: String::from("username"),
                name: Some(String::from("name")),
            }),
            branch: Some(String::from("branch")),
            totals: Totals {
                files: Some(1),
                lines: Some(1),
                hits: Some(1),
                misses: Some(1),
                partials: Some(1),
                coverage: Some(2.0),
                branches: Some(1),
                methods: Some(1),
                sessions: Some(1),
                complexity: Some(1.0),
                complexity_total: Some(1.0),
                complexity_ratio: Some(1.0),
                diff: Some(1),
            },
            state: Some(String::from("state")),
            parent: Some(String::from("parent")),
        };
        response.results.push(commit);
        assert_eq!(response.coverage(), Some(2.0));
    }
}
