use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use regex::RegexBuilder;

#[derive(Serialize, Deserialize)]
pub struct MatchResult {
    pub line_number: usize,
    pub text: String,
}

#[wasm_bindgen]
pub fn grep_search(text: &str, pattern: &str, ignore_case: bool) -> Result<JsValue, JsValue> {
    let re = RegexBuilder::new(pattern)
        .case_insensitive(ignore_case)
        .build()
        .map_err(|e| JsValue::from_str(&format!("Invalid Regex: {}", e)))?;

    let mut results: Vec<MatchResult> = Vec::new();

    for (idx, line) in text.lines().enumerate() {
        if re.is_match(line) {
            results.push(MatchResult {
                line_number: idx + 1,
                text: line.to_string(),
            });
            if results.len() >= 2000 {
                break;
            }
        }
    }

    serde_wasm_bindgen::to_value(&results)
        .map_err(|e| JsValue::from_str(&format!("Serialize Error: {}", e)))
}
