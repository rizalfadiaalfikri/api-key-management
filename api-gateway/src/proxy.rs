use axum::{body::{Body, to_bytes}, extract::{Request}, response::{IntoResponse, Response}};
use http::{HeaderMap, Method, StatusCode};
use reqwest::Url;
use tracing::{error, info};

use crate::state::{AppState};

pub async fn forward_request(
    axum::extract::State(state): axum::extract::State<AppState>,
    req: Request<Body>,
) -> impl IntoResponse {
    match proxy_request(&state, req).await {
        Ok(response) => response,
        Err((status, message)) => {
            error!("proxy error: {} - {}", status, message);
            (status, message).into_response()
        }
    }
}

pub async fn proxy_request(
    state: &AppState,
    req: Request<Body>
) -> Result<Response<Body>, (StatusCode, String)> {
    let (parts, body) = req.into_parts();

    let method = parts.method.clone();
    let headers = parts.headers.clone();
    let path = parts.uri.path().to_string();
    let query = parts.uri.query().unwrap_or("");

    let target_base = resolve_target_base(&state, &path)
        .ok_or_else(|| (StatusCode::NOT_FOUND, format!("route tidak dikenali: {}", path)))?;

    let target_url = build_target_url(target_base, &path, Some(query).filter(|q| !q.is_empty()))?;

    let body_bytes = to_bytes(body, usize::MAX)
        .await
        .map_err(|err| (StatusCode::BAD_REQUEST, format!("gagal membaca body request: {err}")))?;

    info!("forward {} {} -> {}", method, path, target_url);

    let reqwest_method = to_reqwest_method(&method)?;
    let mut request_builder = state.client.request(reqwest_method, target_url);

    request_builder = copy_request_headers(request_builder, &headers);
    request_builder = request_builder.body(body_bytes.clone());

    let downstream_response = request_builder
        .send()
        .await
        .map_err(|err| (StatusCode::BAD_GATEWAY, format!("downstream service unreachable: {err}")))?;

    let status = downstream_response.status();
    let response_headers = downstream_response.headers().clone();

    let response_bytes = downstream_response
        .bytes()
        .await
        .map_err(|err| (StatusCode::BAD_GATEWAY, format!("gagal membaca response downstream: {err}")))?;

    let mut response_builder = Response::builder().status(status);

    for (name, value) in response_headers.iter() {
        if should_skip_response_header(name.as_str()) {
            continue;
        }
        response_builder = response_builder.header(name, value);
    }

    response_builder
        .body(Body::from(response_bytes))
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, format!("gagal membangun response: {err}")))
}


fn resolve_target_base<'a>(
    state: &'a AppState,
    path: &str,
) -> Option<&'a str> {
    if path.starts_with("/api/auth") {
        Some(&state.config.auth_url)
    } else if path.starts_with("/api/users") {
        Some(&state.config.user_url)
    } else if path.starts_with("/api/keys") {
        Some(&state.config.key_url)
    } else if path.starts_with("/api/logs") {
        Some(&state.config.log_url)
    } else {
        None
    }
}

fn build_target_url(
    base: &str,
    path: &str,
    query: Option<&str>,
) -> Result<Url, (StatusCode, String)> {
    let normalized_base = base.trim_end_matches('/');

    let mut full = format!("{normalized_base}{path}");

    if let Some(q) = query {
        full.push('?');
        full.push_str(q);
    }

    Url::parse(&full)
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, format!("target url tidak valid: {err}")))
}

fn to_reqwest_method(method: &Method) -> Result<reqwest::Method, (StatusCode, String)> {
    reqwest::Method::from_bytes(method.as_str().as_bytes())
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, format!("method tidak valid: {err}")))
}

fn copy_request_headers(
    mut builder: reqwest::RequestBuilder,
    headers: &HeaderMap,
) -> reqwest::RequestBuilder {
    for (name, value) in headers.iter() {
        if should_skip_request_header(name.as_str()) {
            continue;
        }
        builder = builder.header(name, value);
    }
    builder
}

fn should_skip_request_header(name: &str) -> bool {
    matches!(
        name.to_ascii_lowercase().as_str(),
        "host" | "content-length"
    )
}

fn should_skip_response_header(name: &str) -> bool {
    matches!(
        name.to_ascii_lowercase().as_str(),
        "content-length" | "transfer-encoding" | "connection"
    )
}