use serde::{Deserialize, Serialize};

use crate::totals::Totals;

/**
 * BranchDetailAPIResponse is an enum wrapping all possible responses from the branches API.
 */
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum BranchDetailAPIResponse {
    Success(Box<BranchDetailAPISuccessResponse>),
    NotFound(BranchNotFound),
}

/**
 * BranchesAPIResponse is a struct that represents the response from the branches API.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct BranchDetailAPISuccessResponse {
    pub head_commit: HeadCommit,
    pub name: String,
    pub updatestamp: String, // TODO: ISO Date
}

/**
 * BranchNotFound is a struct that represents a branch not found error.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct BranchNotFound {
    pub detail: String,
}

/**
 * Branch is a struct that represents a branch.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct HeadCommit {
    pub author: Author,
    pub branch: String,
    pub ci_passed: Option<bool>,
    pub commitid: String,
    pub message: String,
    pub parent: Option<String>,
    pub report: Report,
    pub state: String,
    pub timestamp: String, // TODO: ISO Date
    pub totals: Totals,
}

/**
 * Author is a struct that represents an author.
 * Note: username is an optional field.
 * if the author is a bot, username will be null.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct Author {
    pub name: String,
    pub service: String,
    pub username: Option<String>,
}

/**
 * Report is a struct that represents a report.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct Report {
    pub files: Vec<File>,
    pub totals: Totals,
}

/**
 * File is a struct that represents a file.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub name: String,
    pub totals: Totals,
}

impl BranchDetailAPIResponse {
    /**
     * Returns the latest coverage for a branch.
     */
    pub fn latest_coverage(&self) -> f64 {
        match self {
            BranchDetailAPIResponse::Success(branch_detail) => branch_detail.latest_coverage(),
            BranchDetailAPIResponse::NotFound(_) => 0.0,
        }
    }
}

impl BranchDetailAPISuccessResponse {
    /**
     * Returns the latest coverage for a branch.
     */
    pub fn latest_coverage(&self) -> f64 {
        self.head_commit.totals.coverage
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_deserialize_branch_detail_api_response() {
        let j = json!({
            "head_commit": {
                "author": {
                    "name": "renovate[bot]",
                    "service": "github",
                    "username": null
                },
                "branch": "main",
                "ci_passed": true,
                "commitid": "1eb341765e7c3daa88ae5d2a751538a620c6dbce",
                "message": "chore(deps): update dependency @swc/core to v1.3.73",
                "parent": "5a4b2987ca3a8a7b54efac914fd72455ebff50ba",
                "report": {
                    "files": [],
                    "totals": {
                        "branches": 22,
                        "complexity": 0.0,
                        "complexity_ratio": 0,
                        "complexity_total": 0.0,
                        "coverage": 86.05,
                        "diff": 0,
                        "files": 10,
                        "hits": 148,
                        "lines": 172,
                        "messages": 0,
                        "methods": 0,
                        "misses": 23,
                        "partials": 1,
                        "sessions": 1
                    }
                },
                "state": "complete",
                "timestamp": "2023-08-01T15:41:47Z",
                "totals": {
                    "branches": 22,
                    "complexity": 0.0,
                    "complexity_ratio": 0,
                    "complexity_total": 0.0,
                    "coverage": 86.05,
                    "diff": 0,
                    "files": 10,
                    "hits": 148,
                    "lines": 172,
                    "methods": 0,
                    "misses": 23,
                    "partials": 1,
                    "sessions": 1
                }
            },
            "name": "main",
            "updatestamp": "2023-08-01T19:10:56.045522Z"
        });
        serde_json::from_value::<BranchDetailAPIResponse>(j).unwrap();
    }

    #[test]
    fn test_deserialize_report() {
        let j = json!({
            "files": [
                {
                    "name": "src/something.ts",
                    "totals": {
                        "branches": 1,
                        "complexity": 0.0,
                        "complexity_ratio": 0,
                        "complexity_total": 0.0,
                        "coverage": 0.0,
                        "diff": 0,
                        "files": 0,
                        "hits": 0,
                        "lines": 12,
                        "messages": 0,
                        "methods": 0,
                        "misses": 12,
                        "partials": 0,
                        "sessions": 0
                    }
                },
            ],
            "totals": {
                "branches": 22,
                "complexity": 0.0,
                "complexity_ratio": 0,
                "complexity_total": 0.0,
                "coverage": 86.05,
                "diff": 0,
                "files": 10,
                "hits": 148,
                "lines": 172,
                "messages": 0,
                "methods": 0,
                "misses": 23,
                "partials": 1,
                "sessions": 1
            }
        });
        serde_json::from_value::<Report>(j).unwrap();
    }

    #[test]
    fn test_deserialize_bug_regression() {
        let j = json!({
            "head_commit": {
                "author": {
                    "name": "renovate[bot]",
                    "service": "github",
                    "username": null
                },
                "branch": "main",
                "ci_passed": true,
                "commitid": "1eb341765e7c3daa88ae5d2a751538a620c6dbce",
                "message": "chore(deps): update dependency @swc/core to v1.3.73",
                "parent": null,
                "report": {
                    "files": [],
                    "totals": {
                        "branches": 22,
                        "complexity": 0.0,
                        "complexity_ratio": 0,
                        "complexity_total": 0.0,
                        "coverage": 86.05,
                        "diff": 0,
                        "files": 10,
                        "hits": 148,
                        "lines": 172,
                        "messages": 0,
                        "methods": 0,
                        "misses": 23,
                        "partials": 1,
                        "sessions": 1
                    }
                },
                "state": "complete",
                "timestamp": "2023-08-01T15:41:47Z",
                "totals": {
                    "branches": 22,
                    "complexity": 0.0,
                    "complexity_ratio": 0,
                    "complexity_total": 0.0,
                    "coverage": 86.05,
                    "diff": 0,
                    "files": 10,
                    "hits": 148,
                    "lines": 172,
                    "methods": 0,
                    "misses": 23,
                    "partials": 1,
                    "sessions": 1
                }
            },
            "name": "main",
            "updatestamp": "2023-08-01T19:10:56.045522Z"
        });
        serde_json::from_value::<BranchDetailAPIResponse>(j).unwrap();
    }
}
