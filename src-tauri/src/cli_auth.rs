use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::thread::sleep;
use std::time::{Duration, Instant};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct CreateSessionPayload<'a> {
    server_url: &'a str,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSessionResponse {
    pub session_id: String,
    pub auth_url: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "status", rename_all = "lowercase")]
enum PollResponse {
    Pending,
    Expired,
    Approved { token: String },
}

pub fn create_auth_session(base_url: &str) -> Result<CreateSessionResponse, String> {
    let client = Client::new();
    let trimmed = base_url.trim_end_matches('/');
    let response = client
        .post(format!("{trimmed}/api/cli/auth/session"))
        .json(&CreateSessionPayload { server_url: trimmed })
        .send()
        .map_err(|error| format!("auth session request failed: {error}"))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().unwrap_or_default();
        return Err(format!("auth session failed with status {status}: {body}"));
    }

    response
        .json::<CreateSessionResponse>()
        .map_err(|error| format!("failed to decode auth session response: {error}"))
}

pub fn poll_for_token(
    base_url: &str,
    session_id: &str,
    timeout: Duration,
) -> Result<String, String> {
    let client = Client::new();
    let trimmed = base_url.trim_end_matches('/');
    let started = Instant::now();

    loop {
        if started.elapsed() > timeout {
            return Err("sign-in timed out — click Sign in to try again".into());
        }

        let response = client
            .get(format!("{trimmed}/api/cli/auth/poll"))
            .query(&[("session_id", session_id)])
            .send()
            .map_err(|error| format!("poll request failed: {error}"))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            return Err(format!("poll failed with status {status}: {body}"));
        }

        let parsed: PollResponse = response
            .json()
            .map_err(|error| format!("failed to decode poll response: {error}"))?;

        match parsed {
            PollResponse::Approved { token } => return Ok(token),
            PollResponse::Expired => {
                return Err("sign-in session expired — click Sign in to try again".into());
            }
            PollResponse::Pending => sleep(Duration::from_millis(1500)),
        }
    }
}
