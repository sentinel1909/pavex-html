// app/src/routes/storage_check.rs

// dependencies
use crate::configuration::AppConfig;
use pavex::{request::path::PathParams, response::{body::Json, Response}};
use std::borrow::Cow;

#[PathParams]
pub struct DirectoryPathParam {
  pub dir: Cow<'static, str>,
}

#[derive(serde::Serialize)]
struct StorageCreateResponse {
  message: Cow<'static, str>,
}

// storage_create get handler function
pub async fn get(config: &AppConfig, params: PathParams<DirectoryPathParam>) -> Response {

  let directory = params.0.dir;
  let storage = config.local_storage_config().build().unwrap();

  storage.create_dir(directory.as_ref()).await.unwrap();

  let response = StorageCreateResponse {
    message: Cow::Borrowed("Created"),
  };

  let json_response = Json::new(response).expect("Failed to serialize the response body.");

  Response::ok().set_typed_body(json_response)
}
