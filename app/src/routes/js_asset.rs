// src/routes/js_asset.rs

// dependencies
use crate::asset::JSAsset;
use pavex::http::HeaderValue;
use pavex::response::body::{
    TypedBody,
    raw::{Full, Bytes}
};
use pavex::response::Response;

// implement the TypedBody trait for the JSAsset type, so that the 'application/javascript' header is set
impl TypedBody for JSAsset {
    type Body = Full<Bytes>;

    fn content_type(&self) -> HeaderValue {
        HeaderValue::from_static("application/javascript")
    }

    fn body(self) -> Self::Body {
        Full::new(self.0.into())
    }
}

// handler function which responds with a 200 OK and the JS script file
pub fn get(asset: &JSAsset) -> Response {
    Response::ok().set_typed_body(asset.clone())
}