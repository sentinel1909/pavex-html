// tests/integration/index.rs

use crate::helpers::TestApi;
use pavex::http::StatusCode;

#[tokio::test]
async fn index_works() {
    let api = TestApi::spawn().await;

    let response = api.get_index().await;

    assert_eq!(response.status(), StatusCode::OK);

    let response_header = response
        .headers()
        .get("Content-Type")
        .expect("Expected Content-Type header in response.");

    let response_header_str = response_header
        .to_str()
        .expect("Unable to get the response header text");

    let expected_header_str = "text/html; charset=utf-8";

    assert_eq!(response_header_str, expected_header_str);

    let response_body = response.text().await.unwrap();

    assert!(
        response_body.contains("<!DOCTYPE html>"),
        "Response body does not contain expected HTML."
    )
}
