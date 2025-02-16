// app/src/assets.rs

// dependencies
use rust_embed::Embed;

// struct type to represent an asset
#[derive(Embed)]
#[folder = "../static"]
pub struct Asset;

// struct type to represent a CSS asset
#[derive(Clone)]
pub struct CSSAsset(pub Vec<u8>);

// implementation block for the CSSAsset type (registered as a constructor for Pavex)
impl CSSAsset {
    pub fn build_css_asset() -> Self {
        let css_asset = Asset::get("screen.css").unwrap();

        Self(css_asset.data.to_vec())
    }
}