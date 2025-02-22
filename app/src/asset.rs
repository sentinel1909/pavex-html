// app/src/asset.rs

// dependencies
use mime_guess::from_path;
use pavex::http::{header::InvalidHeaderValue, HeaderValue};
use pavex::request::path::PathParams;
use pavex::response::Response;
use rust_embed_for_web::{EmbedableFile, RustEmbed};
use std::borrow::Cow;

// trait which helps define a generic asset provider
pub trait AssetProvider {
    fn get_asset(&self, filename: &str) -> Option<Vec<u8>>;
}

// struct type for an embedded asset
pub struct EmbeddedAsset;

// implement the AssetP:rovider trait for EmbeddedAsset
impl AssetProvider for EmbeddedAsset {
    fn get_asset(&self, filename: &str) -> Option<Vec<u8>> {
        Asset::get(filename).map(|f| f.data())
    }
}

// struct type to represent a static asset from the file system
#[derive(RustEmbed)]
#[folder = "../static"]
struct Asset;

// struct type to represent incoming path parameters
#[PathParams]
pub struct GetFilenameParams<'a> {
    filename: Cow<'a, str>,
}

// struct type to represent a static asset, CSS, JS, an image, or anything else
#[derive(Debug, Clone)]
pub struct StaticAsset {
    pub asset_name: Cow<'static, str>,
    pub asset_data: Vec<u8>,
    pub asset_mime_type: Cow<'static, str>,
    pub asset_header_value: HeaderValue,
}
// error handler for build static asset constructor
pub async  fn invalid_header2response(e: &pavex::Error) -> Response {
    Response::internal_server_error().set_typed_body(e.to_string())
}

// methods for the StaticAsset type
impl StaticAsset {
    pub fn build_static_asset(params: PathParams<GetFilenameParams>) -> Result<Self, InvalidHeaderValue> {
        let asset_file = params.0.filename;

        let asset_name = Cow::Owned(asset_file.to_string());

        let asset_mime_type = from_path(asset_file.as_ref())
            .first_raw()
            .map(|s| Cow::Owned(s.to_string()))
            .unwrap_or_else(|| Cow::Owned("application/octet-stream".to_string()));

        let asset_data = EmbeddedAsset.get_asset(asset_file.as_ref()).unwrap_or_default();

        let asset_header_value = HeaderValue::from_str(&asset_mime_type)?;

        Ok(Self {
            asset_name,
            asset_data,
            asset_mime_type,
            asset_header_value,
        })
    }
}