use crate::helpers::TestApi;
use pavex::http::StatusCode;
use serde_json::{json, Value};
use std::borrow::Cow;
use uuid::Uuid;

#[derive(Debug, serde::Serialize)]
struct StorageCreateResponse {
    message: Cow<'static, str>,
}

#[tokio::test]
async fn storage_create_works() {
    let api = TestApi::spawn().await;

    let test_dir = format!("test_{}", Uuid::new_v4());

    let response = api.post_storage_create(&test_dir).await;

    assert_eq!(response.status(), StatusCode::OK);

    let mut test_directory = test_dir.to_string();
    test_directory.push('/');

    let expected_body = StorageCreateResponse {
        message: Cow::Owned(format!("Created new directory named: {}", test_directory)),
    };

    let expected_body_json = json!(expected_body);

    assert_eq!(response.json::<Value>().await.unwrap(), expected_body_json);
}
