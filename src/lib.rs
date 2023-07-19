pub mod author;
pub mod branch_detail;
pub mod branches;
pub mod commits;
pub mod errors;
pub mod repos;
pub mod totals;
pub mod url;
use author::Author;

use crate::errors::Error;

/**
 * Client is a struct that represents a client to the Codecov API.
 */
pub struct Client {
    token: String,
}

/**
 * Owner is a struct that represents an owner of repos.
 */
pub struct Owner {
    service: String,
    username: String,
}

impl Client {
    pub fn new_from_env() -> Result<Client, Error> {
        let token = match std::env::var("CODECOV_OWNER_TOKEN") {
            Ok(token) => token,
            Err(e) => return Err(Error::EnvError(e)),
        };
        Ok(Client::new(token))
    }

    pub fn new(token: String) -> Client {
        Client { token }
    }

    fn auth_header_val(&self) -> String {
        format!("bearer {}", self.token)
    }

    fn owner_endpoint(&self, owner: &Owner) -> String {
        let api_endpoint = "https://codecov.io/api/v2";
        format!("{}/{}/{}", api_endpoint, owner.service, owner.username)
    }

    fn repos_endpoint(&self, author: &author::Author) -> String {
        let owner_endpoint = self.owner_endpoint(&Owner {
            service: author.service.clone(),
            username: author.username.clone(),
        });
        format!("{}/repos/{}", owner_endpoint, author.name)
    }

    /**
     * get_all_repos returns a list of all repos for a given owner.
     * /repos endpoint returns a list of repos for a given owner with pagination.
     * This function will make multiple requests to get all repos.
     */
    pub fn get_all_repos(&self, owner: &Owner) -> Result<Vec<repos::Repo>, Error> {
        let mut repos = Vec::new();
        let mut url = format!("{}/repos?page_size=100", self.owner_endpoint(owner));
        loop {
            let mut repo_list = self.get_repos_page(&url)?;
            match &mut repo_list {
                repos::ReposAPIResponse {
                    results,
                    next: Some(next_url),
                    ..
                } => {
                    repos.append(results);
                    url = next_url.to_string();
                    continue;
                }
                repos::ReposAPIResponse {
                    results,
                    next: None,
                    ..
                } => {
                    repos.append(results);
                    break;
                }
            }
        }
        Ok(repos)
    }

    /**
     * get_repos_page returns a single page of repos.
     * This is a helper function for get_all_repos.
     */
    fn get_repos_page(&self, url: &str) -> Result<repos::ReposAPIResponse, Error> {
        self.api_request::<repos::ReposAPIResponse>(url)
    }

    /**
     * api_raw_json returns a serde_json::Value from a given url.
     */
    fn api_raw_json(&self, url: &str) -> Result<serde_json::Value, Error> {
        let client = reqwest::blocking::Client::new();
        let req = client
            .get(url)
            .header("Authorization", self.auth_header_val());
        let res = match req.send() {
            Ok(res) => res,
            Err(e) => return Err(Error::ReqwestError(e)),
        };
        let res = match res.json::<serde_json::Value>() {
            Ok(res) => res,
            Err(e) => return Err(Error::ReqwestError(e)),
        };
        Ok(res)
    }

    /**
     * api_request returns a deserialized struct from a given url.
     */
    fn api_request<T: serde::de::DeserializeOwned + std::fmt::Debug>(
        &self,
        url: &str,
    ) -> Result<T, Error> {
        let res = self.api_raw_json(url)?;
        let data = match serde_json::from_value::<T>(res) {
            Ok(data) => data,
            Err(e) => return Err(Error::DeserializeError(e)),
        };
        Ok(data)
    }

    /**
     * get_commits returns a list of commits for a given author.
     * https://docs.codecov.com/reference/repos_commits_list
     */
    pub fn get_commits(
        &self,
        author: &author::Author,
    ) -> Result<commits::CommitsAPIResponse, Error> {
        let url = format!("{}/commits", self.repos_endpoint(author));
        let commits = self.api_request::<commits::CommitsAPIResponse>(&url)?;
        Ok(commits)
    }

    /**
     * get_branches returns a list of branches for a given author.
     * https://docs.codecov.com/reference/repos_branches_list
     */
    pub fn get_branches(
        &self,
        author: &author::Author,
    ) -> Result<branches::BranchesAPIResponse, Error> {
        let url = format!("{}/branches", self.repos_endpoint(author));
        let branches = self.api_request::<branches::BranchesAPIResponse>(&url)?;
        Ok(branches)
    }

    /**
     * get_branch_detail returns a branch detail for a given author and branch name.
     * https://docs.codecov.com/reference/repos_branches_retrieve
     */
    pub fn get_branch_detail(
        &self,
        author: &Author,
        branch_name: &str,
    ) -> Result<branch_detail::BranchDetailAPIResponse, Error> {
        let url = format!("{}/branches/{}", self.repos_endpoint(author), branch_name);
        let branch_detail = self.api_request::<branch_detail::BranchDetailAPIResponse>(&url)?;
        Ok(branch_detail)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all_repos() {
        let client = Client::new_from_env().unwrap();
        let owner = Owner {
            service: "github".to_string(),
            username: "codecov".to_string(),
        };
        let repos = client.get_all_repos(&owner).unwrap();
        assert!(!repos.is_empty());
    }

    #[test]
    fn test_get_commits() {
        let client = Client::new_from_env().unwrap();
        let author = author::Author {
            service: "github".to_string(),
            username: "codecov".to_string(),
            name: "codecov-demo".to_string(),
        };
        let commits = client.get_commits(&author).unwrap();
        assert!(!commits.results.is_empty());
    }

    #[test]
    fn test_get_branches() {
        let client = Client::new_from_env().unwrap();
        let author = author::Author {
            service: "github".to_string(),
            username: "codecov".to_string(),
            name: "codecov-demo".to_string(),
        };
        let branches = client.get_branches(&author).unwrap();
        assert!(!branches.results.is_empty());
    }

    #[test]
    fn test_get_branch_detail() {
        let client = Client::new_from_env().unwrap();
        let author = author::Author {
            service: "github".to_string(),
            username: "codecov".to_string(),
            name: "codecov-demo".to_string(),
        };
        let branch_name = "main";
        let branch_detail = client.get_branch_detail(&author, branch_name).unwrap();
        assert_eq!(branch_detail.name, branch_name);
    }
}
