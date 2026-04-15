pub fn normalize_focus_runtime_error(message: &str) -> String {
    let lowered = message.to_ascii_lowercase();

    if lowered.contains("status 401")
        || lowered.contains("unauthorized")
        || lowered.contains("status 410")
        || lowered.contains("expired")
        || lowered.contains("revoked")
    {
        return "Sign-in is no longer valid. Click Sign in to reconnect.".into();
    }

    if lowered.contains("status 429") || lowered.contains("too many requests") {
        return "Sign-in or upload is temporarily rate-limited. Wait a few minutes and try again.".into();
    }

    message.to_string()
}

#[cfg(test)]
mod tests {
    use super::normalize_focus_runtime_error;

    #[test]
    fn rewrites_invalid_token_errors_into_reconnect_guidance() {
        assert_eq!(
            normalize_focus_runtime_error("status sync failed with status 401 Unauthorized"),
            "Sign-in is no longer valid. Click Sign in to reconnect."
        );
        assert_eq!(
            normalize_focus_runtime_error("poll failed with status 410: session expired"),
            "Sign-in is no longer valid. Click Sign in to reconnect."
        );
    }

    #[test]
    fn rewrites_rate_limit_errors_into_retry_guidance() {
        assert_eq!(
            normalize_focus_runtime_error("upload failed with status 429: Too Many Requests"),
            "Sign-in or upload is temporarily rate-limited. Wait a few minutes and try again."
        );
    }
}
