// app/src/asset.rs

// dependencies
use mime_guess::from_path;
use pavex::request::path::PathParams;
use rust_embed_for_web::{EmbedableFile, RustEmbed};
use std::borrow::Cow;

// struct type to represent a static asset from the file system
#[derive(RustEmbed)]
#[folder = "../static"]
struct Asset;

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
}

// methods for the StaticAsset type
impl StaticAsset {
    pub fn build_static_asset(params: PathParams<GetFilenameParams>) -> Self {
        let file = params.0.filename;
        Self {
            name: Cow::Owned(file.to_string()),
            data: Asset::get(&file).unwrap().data(),
            mime_type: from_path(file.as_ref()) 
                .first_raw()
                .map(|s| Cow::Owned(s.to_string()))
                .unwrap_or_else(|| Cow::Borrowed("application/octet-stream")),
        }
    }
}