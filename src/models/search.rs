use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    pub total: u64,
    pub hits: Vec<CrashHit>,
    #[serde(default)]
    pub facets: HashMap<String, Vec<FacetBucket>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CrashHit {
    pub uuid: String,
    pub date: String,
    pub signature: String,
    pub product: String,
    pub version: String,
    #[serde(default)]
    pub os_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FacetBucket {
    pub term: String,
    pub count: u64,
}

pub struct SearchParams {
    pub signature: Option<String>,
    pub product: String,
    pub version: Option<String>,
    pub platform: Option<String>,
    pub days: u32,
    pub limit: usize,
    pub facets: Vec<String>,
    pub sort: String,
}
