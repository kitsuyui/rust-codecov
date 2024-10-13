pub mod author;
pub mod branch_detail;
pub mod branches;
pub mod client;
pub mod commits;
pub mod errors;
pub mod owner;
pub mod repos;
pub mod totals;
pub mod url;

/**
```
use codecov::{Client, owner::Owner};

// let client = Client::new("1234-5678-9012-3456"); // Set token directly
let client = Client::new_from_env().unwrap();  // Read CODECOV_OWNER_TOKEN from environment variable
let owner = Owner::new("github", "kitsuyui");
let repos = client.get_all_repos(&owner).unwrap();
println!("{:?}", repos.len());

let author = owner.new_author("rust-codecov");
let repo_detail = client.get_branch_detail(&author, "main").unwrap();
println!("{:?}", repo_detail);
println!("{}", repo_detail.latest_coverage());
```
 */
pub use client::Client;
