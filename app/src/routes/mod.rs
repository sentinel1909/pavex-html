pub mod index;
pub mod ping;
pub mod static_assets;
pub mod storage_create;

use pavex::blueprint::{router::GET, router::POST, Blueprint};
use pavex::f;

pub fn register(bp: &mut Blueprint) {
    bp.route(GET, "/", f!(self::index::get))
        .error_handler(f!(crate::routes::index::tera_error2response));
    bp.route(GET, "/static/{filename}", f!(self::static_assets::get));
    bp.route(GET, "/api/ping", f!(self::ping::get));
    bp.route(POST, "/api/storage/create", f!(self::storage_create::post))
        .error_handler(f!(crate::routes::storage_create::storage_create2response));
}
