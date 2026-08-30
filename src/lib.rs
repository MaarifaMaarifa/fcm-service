use std::error::Error;

use gcp_auth::{CustomServiceAccount, TokenProvider};
use reqwest::Client;
use serde::{Deserialize, Serialize};
mod domain;

pub use domain::{
    AndroidConfig, AndroidNotification, ApnsConfig, Color, FcmMessage, FcmNotification, FcmOptions,
    LightSettings, NotificationPriority, Priority, Proxy, Target, Visibility, WebpushConfig,
};

/// Wrapper struct for FCM payload, required by the FCM v1 API.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct FcmPayload {
    pub message: FcmMessage,
}

pub struct FcmService {
    client: Client,
    custom_service_account: CustomServiceAccount,
    firebase_notification_endpoint: String,
}

impl FcmService {
    pub async fn new(
        credential_file_path: impl AsRef<std::path::Path>,
    ) -> Result<Self, Box<dyn Error>> {
        let credential_file_content = tokio::fs::read_to_string(credential_file_path).await?;

        Ok(Self {
            client: Client::new(),
            custom_service_account: CustomServiceAccount::from_json(&credential_file_content)?,
            firebase_notification_endpoint: Self::construct_firebase_notification_endpoint(
                &credential_file_content,
            )?,
        })
    }

    /// Constructs the Firebase Cloud Messaging (FCM) HTTP v1 endpoint
    /// using the `project_id` found in a Firebase service account credential file.
    fn construct_firebase_notification_endpoint(
        credential_file_content: &str,
    ) -> Result<String, serde_json::Error> {
        #[derive(Deserialize)]
        struct FirebaseCredentials<'a> {
            project_id: &'a str,
        }

        let credentials: FirebaseCredentials = serde_json::from_str(credential_file_content)?;

        if credentials.project_id.is_empty() {
            return Err(serde::de::Error::custom("project_id must not be empty"));
        }

        Ok(format!(
            "https://fcm.googleapis.com/v1/projects/{}/messages:send",
            credentials.project_id
        ))
    }
}

/// Service for sending Firebase Cloud Messaging (FCM) notifications using the v1 API.
///
/// This service uses a Google Cloud service account credential file to authenticate
/// and send notifications to FCM.
///
/// # Examples
/// ```rust,no_run
/// use fcm_service::{FcmService, FcmMessage, FcmNotification, Target};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let service = FcmService::new("path/to/service-account.json").await?;
///
///     let mut message = FcmMessage::new();
///     let mut notification = FcmNotification::new();
///     notification.set_title("Hello".to_string());
///     notification.set_body("World".to_string());
///     notification.set_image(None);
///     message.set_notification(Some(notification));
///     message.set_target(Target::Token("device-token".to_string()));
///
///     service.send_notification(message).await?;
///     Ok(())
/// }
/// ```
impl FcmService {
    /// Sends an FCM notification asynchronously.
    ///
    /// # Errors
    /// Returns an error if:
    /// - Authentication with GCP fails
    /// - The HTTP request to FCM fails
    /// - The FCM API returns an unsuccessful status
    pub async fn send_notification(&self, message: FcmMessage) -> Result<(), Box<dyn Error>> {
        let scopes = &["https://www.googleapis.com/auth/firebase.messaging"];
        let token = self.custom_service_account.token(scopes).await?;

        let payload = FcmPayload { message };

        let response = self
            .client
            .post(&self.firebase_notification_endpoint)
            .bearer_auth(token.as_str())
            .json(&payload)
            .send()
            .await?;

        if response.status().is_success() {
            response.text().await?;

            Ok(())
        } else {
            let error_text = response.text().await?;
            Err(format!("Failed to send notification: {error_text:#?}").into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_firebase_notification_endpoint_success() {
        let json = r#"{
            "project_id": "test-project"
        }"#;

        let endpoint = FcmService::construct_firebase_notification_endpoint(json).unwrap();

        assert_eq!(
            endpoint,
            "https://fcm.googleapis.com/v1/projects/test-project/messages:send"
        );
    }

    #[test]
    fn test_construct_firebase_notification_endpoint_missing_project_id() {
        let json = r#"{
            "client_email": "test@example.com"
        }"#;

        let result = FcmService::construct_firebase_notification_endpoint(json);

        assert!(result.is_err());
    }

    #[test]
    fn test_construct_firebase_notification_endpoint_invalid_json() {
        let result = FcmService::construct_firebase_notification_endpoint("invalid json");

        assert!(result.is_err());
    }

    #[test]
    fn test_construct_firebase_notification_endpoint_empty_project_id() {
        let json = r#"{
            "project_id": ""
        }"#;

        let result = FcmService::construct_firebase_notification_endpoint(json);

        assert!(result.is_err());
    }
}
