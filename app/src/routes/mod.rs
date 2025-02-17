pub mod css_asset;
pub mod index;
pub mod js_asset;
pub mod ping;

use pavex::blueprint::{router::GET, Blueprint};
use pavex::f;

pub fn register(bp: &mut Blueprint) {
    bp.route(GET, "/", f!(self::index::get))
        .error_handler(f!(crate::routes::index::tera_error2response));
    bp.route(GET, "/api/ping", f!(self::ping::get));
    bp.route(GET,  "/static/screen.css", f!(self::css_asset::get));
    bp.route(GET, "/static/script.js", f!(self::js_asset::get));
}
