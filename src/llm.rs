// use llm::models::Llama;
// use llm::{InferenceSessionConfig, InferenceParameters, LoadProgress};
// use std::path::Path;
// use anyhow::Result;

// pub fn run_local_llm(prompt: &str) -> Result<()> {
//     let model_path = Path::new("models/llama-2-7b.ggml.q4_0.bin");

//     // Load the LLaMA model from the file
//     let llama_model = Llama::load(
//         model_path,
//         llm::TokenizerSource::Embedded,
//         LoadProgress::default(),
//         Default::default(),
//     )?;

//     // Create a session with the model
//     let mut session = llama_model.start_session(InferenceSessionConfig::default());

//     // Run inference
//     let res = session.infer::<std::convert::Infallible>(
//         &llama_model,
//         &mut rand::thread_rng(),
//         &llm::InferenceRequest {
//             prompt: prompt.into(),
//             parameters: &InferenceParameters::default(),
//             play_back_previous_tokens: false,
//             maximum_token_count: Some(100),
//         },
//         &mut Default::default(),
//         |_| {},
//     )?;

//     println!("\nAnswer:\n{}", res.output_text);
//     Ok(())
// }
