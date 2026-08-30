use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Configuration for Apple Push Notification Service (APNs).
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ApnsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payload: Option<HashMap<String, serde_json::Value>>,
}

impl ApnsConfig {
    #[must_use]
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    #[must_use]
    pub fn headers(&self) -> Option<&HashMap<String, String>> {
        self.headers.as_ref()
    }

    #[must_use]
    pub fn payload(&self) -> Option<&HashMap<String, Value>> {
        self.payload.as_ref()
    }

    pub fn set_headers(&mut self, headers: Option<HashMap<String, String>>) {
        self.headers = headers;
    }

    pub fn set_payload(&mut self, payload: Option<HashMap<String, serde_json::Value>>) {
        self.payload = payload;
    }
}
