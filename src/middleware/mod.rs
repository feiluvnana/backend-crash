use axum::{extract::Request, http::HeaderValue, middleware::Next, response::Response};
use tracing::info_span;
use uuid::Uuid;

pub async fn request_id_middleware(mut request: Request, next: Next) -> Response {
    let request_id = request
        .headers()
        .get("x-request-id")
        .and_then(|val| val.to_str().ok().map(|s| s.to_string()))
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    let header_val =
        HeaderValue::from_str(&request_id).unwrap_or_else(|_| HeaderValue::from_static(""));
    request
        .headers_mut()
        .insert("x-request-id", header_val.clone());

    let span = info_span!("request", request_id = %request_id);
    use tracing::Instrument;
    let mut response = next.run(request).instrument(span).await;

    response.headers_mut().insert("x-request-id", header_val);
    response
}
