// tests/integration/static_asset.rs

use crate::helpers::TestApi;
use pavex::http::StatusCode;

#[tokio::test]
async fn static_asset_works() {
    let api = TestApi::spawn().await;

    let filename = "test.css";

    let response = api.get_static_asset(filename).await;

    assert_eq!(response.status(), StatusCode::OK);

    let response_header = response
        .headers()
        .get("Content-Type")
        .expect("Expected Content-Type header in response.");

    let response_header_str = response_header
        .to_str()
        .expect("Unable to get the response header text");

    let expected_header_str = "text/css; charset=utf-8";

    assert_eq!(response_header_str, expected_header_str);
}
