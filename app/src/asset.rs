// app/src/asset.rs

// dependencies
use mime_guess::from_path;
use pavex::request::path::PathParams;
use rust_embed_for_web::{EmbedableFile, RustEmbed};

// struct type to represent a static asset from the file system
#[derive(RustEmbed)]
#[folder = "../static"]
struct Asset;

#[PathParams]
pub struct GetFilenameParams {
    filename: String,
}

// struct type to represent a static asset, CSS, JS, an image, or anything else
#[derive(Debug, Clone)]
pub struct StaticAsset {
    pub name: String,
    pub data: Vec<u8>,
    pub mime_type: &'static str,
}

// methods for the StaticAsset type
impl StaticAsset {
    pub fn build_static_asset(params: PathParams<GetFilenameParams>) -> Self {
        let file = params.0.filename;
        Self {
            name: file.clone(),
            data: Asset::get(file.as_str()).unwrap().data(),
            mime_type: from_path(file.as_str())
                .first_raw()
                .unwrap_or("application/octet-stream"),
        }
    }
}