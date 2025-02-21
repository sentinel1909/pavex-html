// app/src/asset.rs

// dependencies
use mime_guess::from_path;
use pavex::http::HeaderValue;
use pavex::request::path::PathParams;
use rust_embed_for_web::{EmbedableFile, RustEmbed};
use std::borrow::Cow;

// trait for asset retrieval
pub trait AssetProvider {
    fn get_asset(&self, filename: &str) -> Option<Vec<u8>>;
}

// default provider for real assets
pub struct EmbeddedAssetProvider;

// implement the AssetProvider trait for EmbeddedAssetProvider
impl AssetProvider for crate::asset::EmbeddedAssetProvider {
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
    pub name: Cow<'static, str>,
    pub data: Vec<u8>,
    pub mime_type: Cow<'static, str>,
    pub mime_header: HeaderValue,
}

// methods for the StaticAsset type
impl StaticAsset {
    pub fn build_static_asset(params: PathParams<GetFilenameParams>, provider: &dyn crate::asset::AssetProvider) -> Option<Self> {
        let file = params.0.filename;

        let name = Cow::Owned(file.to_string());

        let mime_type = from_path(file.as_ref())
            .first_raw()
            .map(Cow::Borrowed)
            .unwrap_or_else(|| Cow::Borrowed("application/octet-stream"));

        let data = provider.get_asset(file.as_ref())?;

        let mime_header = match HeaderValue::from_str(&mime_type) {
            Ok(hv) => hv,
            Err(_) => HeaderValue::from_static("application/octet-stream"),
        };

        Some(Self {
            name,
            data,
            mime_type,
            mime_header,
        })
    }
}
