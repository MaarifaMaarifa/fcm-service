use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Configuration for web push notifications.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WebpushConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<HashMap<String, String>>,
}

impl WebpushConfig {
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
    pub fn data(&self) -> Option<&HashMap<String, String>> {
        self.data.as_ref()
    }

    pub fn set_headers(&mut self, headers: Option<HashMap<String, String>>) {
        self.headers = headers;
    }

    pub fn set_data(&mut self, data: Option<HashMap<String, String>>) {
        self.data = data;
    }
}
