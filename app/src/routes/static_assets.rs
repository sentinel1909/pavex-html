// src/routes/css_asset.rs

// dependencies
use crate::asset::StaticAsset;
use pavex::http::HeaderValue;
use pavex::response::body::{
    raw::{Bytes, Full},
    TypedBody,
};
use pavex::response::Response;

// implement the TypedBody trait for the CSSAsset type, so that the 'text/css' header is set
impl TypedBody for StaticAsset {
    type Body = Full<Bytes>;

    fn content_type(&self) -> HeaderValue {
        HeaderValue::from_str(&self.mime_type).unwrap()
    }

    fn body(self) -> Self::Body {
        Full::new(self.data.into())
    }
}

// handler function which responds with a 200 OK and the CSS styles
pub fn get(asset: StaticAsset) -> Response {
    Response::ok().set_typed_body(asset)
}
