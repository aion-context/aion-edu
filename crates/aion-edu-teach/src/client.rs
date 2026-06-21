//! A minimal blocking Anthropic Messages client (raw HTTP via `ureq`).
//!
//! Rust has no first-party Anthropic SDK, so we speak the wire protocol
//! directly: `POST /v1/messages` with `x-api-key` + `anthropic-version`.

use serde_json::{json, Value};

use crate::error::{Error, Result};

const ENDPOINT: &str = "https://api.anthropic.com/v1/messages";
const API_VERSION: &str = "2023-06-01";

pub struct Client {
    key: String,
}

impl Client {
    /// Build a client from `ANTHROPIC_API_KEY`.
    pub fn from_env() -> Result<Self> {
        let key = std::env::var("ANTHROPIC_API_KEY")
            .map_err(|_| Error::Api("ANTHROPIC_API_KEY not set".into()))?;
        Ok(Self { key })
    }

    /// One Messages API turn. `tools` is omitted from the body when `None`.
    pub fn message(
        &self,
        model: &str,
        system: &str,
        messages: Value,
        tools: Option<Value>,
        max_tokens: u32,
    ) -> Result<Value> {
        let mut body = json!({
            "model": model,
            "max_tokens": max_tokens,
            "system": system,
            "messages": messages,
        });
        if let Some(t) = tools {
            body["tools"] = t;
        }
        match ureq::post(ENDPOINT)
            .set("x-api-key", &self.key)
            .set("anthropic-version", API_VERSION)
            .set("content-type", "application/json")
            .send_json(body)
        {
            Ok(resp) => Ok(resp.into_json::<Value>()?),
            Err(ureq::Error::Status(code, resp)) => {
                let detail = resp.into_string().unwrap_or_default();
                let detail: String = detail.chars().take(300).collect();
                Err(Error::Api(format!("HTTP {code}: {detail}")))
            }
            Err(e) => Err(Error::Api(e.to_string())),
        }
    }
}

/// Join all `text` content blocks of a response message.
pub fn text_of(message: &Value) -> String {
    message["content"]
        .as_array()
        .map(|blocks| {
            blocks
                .iter()
                .filter(|b| b["type"] == "text")
                .filter_map(|b| b["text"].as_str())
                .collect::<Vec<_>>()
                .join("\n")
        })
        .unwrap_or_default()
}
