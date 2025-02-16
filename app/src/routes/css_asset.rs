// src/routes/css_asset.rs

// dependencies
use crate::asset::CSSAsset;
use pavex::http::HeaderValue;
use pavex::response::body::{
    TypedBody,
    raw::{Full, Bytes}
};
use pavex::response::Response;

// implement the TypedBody trait for the CSSAsset type, so that the 'text/css' header is set
impl TypedBody for CSSAsset {
    type Body = Full<Bytes>;

    fn content_type(&self) -> HeaderValue {
        HeaderValue::from_static("text/css")
    }

    fn body(self) -> Self::Body {
        Full::new(self.0.into())
    }
}

// handler function which responds with a 200 OK and the CSS styles
pub fn get(asset: &CSSAsset) -> Response {
    Response::ok().set_typed_body(asset.clone())
}