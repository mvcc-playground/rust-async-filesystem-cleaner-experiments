use dotenv::dotenv;
use std::env;
#[tokio::main]
async fn main() {
    // pegar da primeira entrada do terminal
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    // ler do dot env
    dotenv().expect("Failed to load .env file");
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not found");

    let url =
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent";

    let client = reqwest::Client::new();

    let response = client
        .post(url)
        .header("x-goog-api-key", api_key)
        .header("Content-Type", "application/json")
        .body(format!(
            "{{\"contents\":[{{\"parts\":[{{\"text\":\"{}\"}}]}}]}}",
            path
        ))
        .send()
        .await
        .unwrap();

    // mostrar tudo da resposta
    let res = response.text().await.unwrap();
    println!("{}", res);
}

struct ResponseGemini {
    candidates: Vec<Candidate>,
    usage_metadata: UsageMetadata,
    model_version: String,
    response_id: String,
}

struct Candidate {
    content: Content,
    finish_reason: String,
    index: u32,
}

struct Content {
    parts: Vec<Part>,
    role: String,
}

struct Part {
    text: String,
}

struct UsageMetadata {
    prompt_token_count: u32,
    candidates_token_count: u32,
    total_token_count: u32,
    prompt_tokens_details: Vec<PromptTokensDetail>,
    thoughts_token_count: u32,
}

struct PromptTokensDetail {
    modality: String,
    token_count: u32,
}

// {
//   "candidates": [
//     {
//       "content": {
//         "parts": [
//           {
//             "text": "Olá! Tudo bem sim, obrigado(a) por perguntar. E você, como está?"
//           }
//         ],
//         "role": "model"
//       },
//       "finishReason": "STOP",
//       "index": 0
//     }
//   ],
//   "usageMetadata": {
//     "promptTokenCount": 3,
//     "candidatesTokenCount": 20,
//     "totalTokenCount": 91,
//     "promptTokensDetails": [
//       {
//         "modality": "TEXT",
//         "tokenCount": 3
//       }
//     ],
//     "thoughtsTokenCount": 68
//   },
//   "modelVersion": "gemini-2.5-flash",
//   "responseId": "5-5Nad_WE_i6qtsPxoeLoQQ"
// }
