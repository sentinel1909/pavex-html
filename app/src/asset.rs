// app/src/asset.rs

// dependencies
use mime_guess::from_path;
use rust_embed_for_web::{EmbedableFile, RustEmbed};

// struct type to represent a static asset from the file system
#[derive(RustEmbed)]
#[folder = "../static"]
struct Asset;

// struct type to represent a static asset, CSS, JS, an image, or anything else
#[derive(Clone)]
pub struct StaticAsset {
    pub name: &'static str,
    pub data: Vec<u8>,
    pub mime_type: &'static str,
}

// methods for the StaticAsset type
impl StaticAsset {
    pub fn build_static_asset() -> Self {
        Self {
            name: "screen.css",
            data: Asset::get("screen.css").unwrap().data().to_vec(),
            mime_type: from_path("screen.css").first_raw().unwrap_or("application/octet-stream"),
        }
    }
}
