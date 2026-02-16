# rust-codecov

[![Crates.io](https://img.shields.io/crates/d/codecov)](https://crates.io/crates/codecov)
[![crates.io](https://img.shields.io/crates/v/codecov.svg)](https://crates.io/crates/codecov)
[![docs.rs](https://docs.rs/codecov/badge.svg)](https://docs.rs/codecov)
[![License: BSD-3-Clause](https://img.shields.io/badge/License-BSD--3--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)

## Description

A thin wrapper for Codecov API (v2).
https://docs.codecov.com/reference/overview

## Archive

This project is archived and no longer maintained because I no longer use Codecov.

## Usage

```rust
use codecov::{Client, owner::Owner};

fn main() {
    // let client = Client::new("1234-5678-9012-3456"); // Set token directly
    let client = Client::new_from_env().unwrap();  // Read CODECOV_OWNER_TOKEN from environment variable
    let owner = Owner::new("github", "kitsuyui");
    let repos = client.get_all_repos(&owner).unwrap();
    println!("{:?}", repos);

    let author = owner.new_author("rust-codecov");
    let repo_detail = client.get_branch_detail(&author, "main").unwrap();
    println!("{:?}", repo_detail);
    println!("{}", repo_detail.latest_coverage());
}
```

## LICENSE

BSD-3-Clause
