use crate::models::{ProcessedCrash, SearchResponse};
use crate::Result;

pub fn format_crash(crash: &ProcessedCrash) -> Result<String> {
    Ok(serde_json::to_string_pretty(crash)?)
}

pub fn format_search(response: &SearchResponse) -> Result<String> {
    Ok(serde_json::to_string_pretty(response)?)
}
