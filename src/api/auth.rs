use axum::http::{header, HeaderMap, StatusCode};
use base64::Engine as _;

use crate::config::AppConfig;

/// When `[DOCS]` has BASIC_USER and BASIC_PASS, require `Authorization: Basic ...`.
pub fn require_docs_if_configured(
    config: &AppConfig,
    headers: &HeaderMap,
) -> Result<(), StatusCode> {
    let (Some(user), Some(pass)) = (
        config.docs.basic_user.as_deref(),
        config.docs.basic_password.as_deref(),
    ) else {
        return Ok(());
    };
    let Some(raw) = headers.get(header::AUTHORIZATION) else {
        return Err(StatusCode::UNAUTHORIZED);
    };
    let Ok(raw) = raw.to_str() else {
        return Err(StatusCode::UNAUTHORIZED);
    };
    let want = base64::engine::general_purpose::STANDARD.encode(format!("{user}:{pass}"));
    let want = format!("Basic {want}");
    if raw != want.as_str() {
        return Err(StatusCode::UNAUTHORIZED);
    }
    Ok(())
}
