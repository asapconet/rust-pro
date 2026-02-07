use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::Value;
use tower::ServiceExt;

use hell_o::errors::api_error::ApiError;
use hell_o::lib::create_app;

#[tokio::test]
async fn test_health_cheak() {
    let app = create_app();

    let request = Request::builder()
        .uri("/health")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap();
    let json: Value = serde_json::from_slice(&body.to_bytes()).unwrap();

    assert_eq!(json["status"], "ok");
    assert_eq!(json["message"], "Server is running");
}

#[tokio::test]
async fn test_api_error_into_response() {
    let test_cases = vec![
        (ApiError::NotFound, StatusCode::NOT_FOUND),
        (
            ApiError::InvalidInput("Wrong input".to_string()),
            StatusCode::BAD_REQUEST,
        ),
        (ApiError::InternalError, StatusCode::INTERNAL_SERVER_ERROR),
    ];
    for (error, expected_status) in test_cases {
        let response = error.into_response();
        assert_eq!(response.status(), expected_status);
    }
}
