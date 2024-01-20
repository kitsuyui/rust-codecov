use serde::{Deserialize, Serialize};

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
    pub diff: Option<Diff>,
}

/**
 * Diff is a struct that represents the diff for a commit.
 * Diff may be a u64 or an array of Option<String> like this:
 * "diff": [0, 0, 0, 0, "81.81818", null, 0, 0, 0, 0, "84.5", null, 0]
 */
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Diff {
    Value(u64),
    Array(Vec<Option<DiffValue>>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum DiffValue {
    NumValue(u64),
    StringValue(String),
}
