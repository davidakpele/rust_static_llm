// use anyhow::Result;

// /// Naive embedding: count word length
// fn embed(text: &str) -> Vec<f32> {
//     text.split_whitespace().map(|w| w.len() as f32).collect()
// }

// fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
//     let dot: f32 = a.iter().zip(b).map(|(x, y)| x * y).sum();
//     let norm_a = a.iter().map(|x| x * x).sum::<f32>().sqrt();
//     let norm_b = b.iter().map(|x| x * x).sum::<f32>().sqrt();
//     dot / (norm_a * norm_b + 1e-5)
// }

// pub fn find_relevant_chunks<'a>(
//     query: &str,
//     chunks: &'a [String],
// ) -> Result<Vec<&'a String>> {
//     let query_embed = embed(query);
//     let mut scored: Vec<(&String, f32)> = chunks
//         .iter()
//         .map(|chunk| (chunk, cosine_similarity(&embed(chunk), &query_embed)))
//         .collect();

//     scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

//     Ok(scored.into_iter().take(3).map(|(chunk, _)| chunk).collect())
// }
