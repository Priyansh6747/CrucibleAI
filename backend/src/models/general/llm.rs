use serde::{Deserialize, Serialize};

///Request Bases Structures
#[derive(Serialize, Deserialize, Debug, Clone ,PartialEq)]
pub struct GeminiContent {
    pub role: String,
    pub parts: Vec<GeminiPart>,
}

#[derive(Serialize, Deserialize, Debug, Clone,PartialEq)]
pub struct GeminiPart {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThinkingConfig {
    #[serde(rename = "thinkingBudget")]
    pub thinking_budget: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GenerationConfig {
    #[serde(rename = "thinkingConfig")]
    pub thinking_config: ThinkingConfig,
    #[serde(rename = "responseMimeType")]
    pub response_mime_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GeminiRequest {
    pub contents: Vec<GeminiContent>,
    #[serde(rename = "generationConfig")]
    pub generation_config: GenerationConfig,
}

impl GeminiContent {
    pub fn new(role:String,content: String) -> GeminiContent  {
        let part = GeminiPart { text: content };
        GeminiContent {
            role,
            parts: vec![part]
        }
    }
}


///Response Based Structures

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeminiResponse {
    pub candidates: Option<Vec<Candidate>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Candidate {
    pub content: Option<Content>,
    #[serde(rename = "finishReason")]
    pub finish_reason: Option<String>,
    pub index: Option<u32>,
    #[serde(rename = "safetyRatings")]
    pub safety_ratings: Option<Vec<SafetyRating>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Content {
    pub parts: Option<Vec<Part>>,
    pub role: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Part {
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SafetyRating {
    pub category: Option<String>,
    pub probability: Option<String>,
}

impl GeminiResponse {

    /// Extract all generated texts from all candidates
    pub fn extract_all_texts(&self) -> Vec<String> {
        let mut texts = Vec::new();

        if let Some(candidates) = &self.candidates {
            for candidate in candidates {
                if let Some(content) = &candidate.content {
                    if let Some(parts) = &content.parts {
                        for part in parts {
                            if let Some(text) = &part.text {
                                texts.push(text.clone());
                            }
                        }
                    }
                }
            }
        }

        texts
    }
    
    pub fn get_string(&self) ->Option<String> {
        let mut out:String = String::new();
        for s in self.extract_all_texts() {
            out.push_str(&s);
            out.push(' ');
        }
        let cleaned = out.trim()
                                .trim_start_matches("```json")
                                .trim_start_matches("```")
                                .trim_end_matches("```")
            .trim();
        Some(cleaned.to_string())
    }
}
