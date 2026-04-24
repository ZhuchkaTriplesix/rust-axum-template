use std::sync::OnceLock;

use axum::extract::State;
use axum::http::HeaderMap;
use axum::http::StatusCode;
use axum::response::{Html, Json};
use axum::routing::get;
use axum::Router;
use serde::Serialize;
use tower_http::trace::TraceLayer;

use super::auth::require_docs_if_configured;
use super::state::AppState;
use crate::domain::Greeting;
use crate::domain::HealthReport;

const OPENAPI_RAW: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/openapi.json"));

static OPENAPI_VALUE: OnceLock<serde_json::Value> = OnceLock::new();

fn openapi_value() -> &'static serde_json::Value {
    OPENAPI_VALUE
        .get_or_init(|| serde_json::from_str(OPENAPI_RAW).expect("embedded openapi.json is valid"))
}

const DOCS_HTML: &str = r#"<!DOCTYPE html>
<html>
<head>
  <meta charset="utf-8"/>
  <title>API</title>
  <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@5/swagger-ui.css"/>
</head>
<body>
  <div id="swagger"></div>
  <script src="https://unpkg.com/swagger-ui-dist@5/swagger-ui-bundle.js" crossorigin></script>
  <script>
    window.onload = () => {
      SwaggerUIBundle({ url: '/api/openapi.json', dom_id: '#swagger' });
    };
  </script>
</body>
</html>
"#;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    database: bool,
    redis: bool,
}

async fn health(State(s): State<AppState>) -> (StatusCode, Json<HealthResponse>) {
    let r: HealthReport = s.health.status().await;
    let all = r.all_ok();
    let code = if all {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };
    let status = if all { "ok" } else { "degraded" };
    (
        code,
        Json(HealthResponse {
            status,
            database: r.database_ok,
            redis: r.redis_ok,
        }),
    )
}

async fn welcome(State(s): State<AppState>) -> Json<Greeting> {
    Json(s.greeting.welcome())
}

async fn openapi_json(
    State(s): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<&'static serde_json::Value>, StatusCode> {
    require_docs_if_configured(&s.config, &headers)?;
    Ok(Json(openapi_value()))
}

async fn docs_page(
    State(s): State<AppState>,
    headers: HeaderMap,
) -> Result<Html<&'static str>, StatusCode> {
    require_docs_if_configured(&s.config, &headers)?;
    Ok(Html(DOCS_HTML))
}

async fn index() -> axum::response::Redirect {
    axum::response::Redirect::permanent("/api/docs")
}

/// Builds the top-level `Router` (health, example use case, OpenAPI, Swagger UI).
pub fn app_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/api/root/health", get(health))
        .route("/api/root/welcome", get(welcome))
        .route("/api/docs", get(docs_page))
        .route("/api/openapi.json", get(openapi_json))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}
