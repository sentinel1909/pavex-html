pub mod index;
pub mod ping;
pub mod static_assets;
pub mod storage_create;

use pavex::blueprint::{router::GET, Blueprint};
use pavex::f;

pub fn register(bp: &mut Blueprint) {
    bp.route(GET, "/", f!(self::index::get))
        .error_handler(f!(crate::routes::index::tera_error2response));
    bp.route(GET, "/static/{filename}", f!(self::static_assets::get));
    bp.route(GET, "/api/ping", f!(self::ping::get));
    bp.route(GET, "/api/storage/create/{dir}", f!(self::storage_create::get));
}
