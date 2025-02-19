// app/src/routes/storage_check.rs

// dependencies
use crate::configuration::AppConfig;
use opendal::Result;
use pavex::{request::query::QueryParams, response::{body::Json, Response}};
use std::borrow::Cow;

#[derive(serde::Deserialize)]
pub struct DirParams {
  pub dir: Cow<'static, str>,
}

#[derive(serde::Serialize)]
struct StorageCreateResponse {
  message: Cow<'static, str>,
}

// error handler
pub async fn storage_create2response(e: &pavex::Error) -> Response {
  Response::internal_server_error().set_typed_body(e.to_string())
}

// storage_create get handler function
pub async fn get(config: &AppConfig, params: &QueryParams<DirParams>) -> Result<Response> {

  let mut directory = params.0.dir.to_string();
  directory.push('/');
  let storage = config.local_storage_config().build()?;

  storage.create_dir(&directory).await?;

  let response = StorageCreateResponse {
    message: Cow::Owned(format!("Created new directory named: {}", directory)),
  };

  let json_response = Json::new(response).expect("Failed to serialize the response body.");

  let response = Response::ok().set_typed_body(json_response);

  Ok(response)
}
