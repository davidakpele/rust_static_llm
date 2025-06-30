// use std::fs;
// use anyhow::Result;

// /// Loads the document and splits it into fixed-length chunks
// pub fn load_document_chunks(path: &str) -> Result<Vec<String>> {
//     let content = fs::read_to_string(path)?;
//     let words: Vec<&str> = content.split_whitespace().collect();
//     let mut chunks = Vec::new();

//     let chunk_size = 100; // words
//     for chunk in words.chunks(chunk_size) {
//         chunks.push(chunk.join(" "));
//     }

//     Ok(chunks)
// }
