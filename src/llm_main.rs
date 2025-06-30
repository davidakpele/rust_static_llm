// mod embed;
// mod search;
// mod llm;

// use clap::Parser;

// #[derive(Parser)]
// struct Args {
//     ///natural language question
//     query: String,
// }

// fn main() -> anyhow::Result<()> {
//     let args = Args::parse();

//     // Load and embed document
//     let chunks = embed::load_document_chunks("docs/guide.txt")?;

//     // Find top 3 relevant chunks
//     let top_chunks = search::find_relevant_chunks(&args.query, &chunks)?;

//     // Build prompt
//     let prompt = format!(
//         "Based on the following:\n{}\n\nAnswer: {}",
//         top_chunks.iter().map(|s| s.as_str()).collect::<Vec<&str>>().join("\n"),
//         args.query
//     );
    
//     // Ask LLM
//     llm::run_local_llm(&prompt)?;

//     Ok(())
// }
