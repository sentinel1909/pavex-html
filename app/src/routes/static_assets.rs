// src/routes/css_asset.rs

// dependencies
use crate::asset::StaticAsset;
use pavex::http::HeaderValue;
use pavex::response::body::{
    raw::{Bytes, Full},
    TypedBody,
};
use pavex::response::Response;

// implement the TypedBody trait for the StaticAsset type, so that the the response body
// can be created
impl TypedBody for StaticAsset {
    type Body = Full<Bytes>;

    fn content_type(&self) -> HeaderValue {
        self.get_header_value()
    }

    fn body(self) -> Self::Body {
        Full::new(self.asset_data.into())
    }
}

// handler function which responds with a 200 OK and the CSS styles
pub fn get(asset: StaticAsset) -> Response {
    Response::ok().set_typed_body(asset)
}
