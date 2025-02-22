// app/src/asset.rs

// dependencies
use mime_guess::from_path;
use pavex::http::HeaderValue;
use pavex::request::path::PathParams;
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
}

// methods for the StaticAsset type
impl StaticAsset {
    pub fn build_static_asset(params: PathParams<GetFilenameParams>) -> Self {
        let asset_file = params.0.filename;

        let asset_name = Cow::Owned(asset_file.to_string());

        let asset_mime_type = from_path(asset_file.as_ref())
            .first_raw()
            .map(|s| Cow::Owned(s.to_string()))
            .unwrap_or_else(|| Cow::Owned("application/octet-stream".to_string()));

        let asset_data = EmbeddedAsset.get_asset(asset_file.as_ref()).unwrap_or_default();

        Self {
            asset_name,
            asset_data,
            asset_mime_type,
        }
    }

    pub fn get_header_value(&self) -> HeaderValue {
        HeaderValue::from_str(&self.asset_mime_type).unwrap_or_else(|_| HeaderValue::from_static("application/octet-stream"))
    }
}