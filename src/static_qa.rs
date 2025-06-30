use serde::Deserialize;
use std::{env, path::PathBuf};

#[derive(Deserialize)]
struct QA {
    question: String,
    answer: String,
}

pub fn find_answer(query: &str) -> Vec<String> {
    let path = get_qa_file_path().unwrap_or_else(|| PathBuf::from("data/qa.json"));
    let data = std::fs::read_to_string(path).unwrap_or_default();
    let qa_list: Vec<QA> = serde_json::from_str(&data).unwrap_or_default();

    let normalized_query = normalize(query);

    // collect all similar matches (e.g., contains keywords)
    qa_list
        .into_iter()
        .filter(|qa| normalize(&qa.question).contains(&normalized_query))
        .map(|qa| qa.answer)
        .collect()
}

fn normalize(s: &str) -> String {
    s.to_lowercase()
        .replace(|c: char| !c.is_alphanumeric() && !c.is_whitespace(), "") // remove punctuation
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn get_qa_file_path() -> Option<PathBuf> {
    let exe_path = env::current_exe().ok()?;
    let exe_dir = exe_path.parent()?;
    let data_path = exe_dir.join("..").join("data").join("qa.json");
    Some(data_path.canonicalize().ok()?)
}
